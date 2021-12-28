use crate::search::*;
use crate::util::*;
use chrono::{DateTime, Utc};
use serde::ser::{Serialize, SerializeMap, Serializer};
use std::fmt::Debug;

/// Each document is scored by the defined functions. The parameter `score_mode` specifies how
/// the computed scores are combined
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FunctionScoreMode {
    /// Scores are multiplied (default)
    Multiply,

    /// Scores are summed
    Sum,

    /// Scores are averaged
    Avg,

    /// The first function that has a matching filter is applied
    First,

    /// Maximum score is used
    Max,

    /// Minimum score is used
    Min,
}

impl Default for FunctionScoreMode {
    fn default() -> Self {
        Self::Multiply
    }
}

/// The newly computed score is combined with the score of the query. The parameter
/// `boost_mode` defines how.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FunctionScoreBoostMode {
    /// Query score and function score is multiplied (default)
    Multiply,

    /// Only function score is used, the query score is ignored
    Replace,

    /// Query score and function score are added
    Sum,

    /// Average
    Avg,

    /// Max of query score and function score
    Max,

    /// Min of query score and function score
    Min,
}

impl Default for FunctionScoreBoostMode {
    fn default() -> Self {
        Self::Multiply
    }
}

macro_rules! function {
    ($name:ident { $($variant:ident($query:ty)),+ $(,)? }) => {
        /// Functions available for use in [FunctionScoreQuery](crate::FunctionScoreQuery)
        #[derive(Debug, Clone, PartialEq, Serialize)]
        #[allow(missing_docs)]
        #[serde(untagged)]
        pub enum $name {
            $(
                $variant($query),
            )*
        }

        $(
            impl From<$query> for $name {
                fn from(q: $query) -> Self {
                    $name::$variant(q)
                }
            }
        )+

        $(
            impl From<$query> for Option<$name> {
                fn from(q: $query) -> Self {
                    Some($name::$variant(q))
                }
            }
        )+
    };
}

function!(Function {
    Weight(Weight),
    RandomScore(RandomScore),
    FieldValueFactor(FieldValueFactor),
    DecayDateTime(Decay<DateTime<Utc>>),
    DecayLocation(Decay<GeoPoint>),
    DecayI8(Decay<i8>),
    DecayI16(Decay<i16>),
    DecayI32(Decay<i32>),
    DecayI64(Decay<i64>),
    DecayU8(Decay<u8>),
    DecayU16(Decay<u16>),
    DecayU32(Decay<u32>),
    DecayU64(Decay<u64>),
    Script(Script),
});

impl Function {
    /// Creates an instance of [Weight](Weight)
    pub fn weight(weight: f32) -> Weight {
        Weight::new(weight)
    }

    /// Creates an instance of [RandomScore](RandomScore)
    pub fn random_score() -> RandomScore {
        RandomScore::new()
    }

    /// Creates an instance of [FieldValueFactor](FieldValueFactor)
    ///
    /// - `field` - Field to be extracted from the document.
    pub fn field_value_factor(field: impl Into<String>) -> FieldValueFactor {
        FieldValueFactor::new(field)
    }

    /// Creates an instance of [Decay](Decay)
    ///
    /// - `function` - Decay function variant
    /// - `field` - Field to apply function to
    /// - `origin` - The point of origin used for calculating distance. Must be given as a number
    /// for numeric field, date for date fields and geo point for geo fields. Required for geo and
    /// numeric field. For date fields the default is `now`. Date math (for example now-1h) is
    /// supported for origin.
    /// - `scale` - Required for all types. Defines the distance from origin + offset at which the
    /// computed score will equal `decay` parameter. For geo fields: Can be defined as number+unit
    /// (1km, 12m,…​). Default unit is meters. For date fields: Can to be defined as a number+unit
    /// ("1h", "10d",…​). Default unit is milliseconds. For numeric field: Any number.
    pub fn decay<T: Origin>(
        function: DecayFunction,
        field: impl Into<String>,
        origin: T,
        scale: <T as Origin>::Scale,
    ) -> Decay<T> {
        Decay::new(function, field, origin, scale)
    }

    /// Creates an instance of [Script](Script)
    ///
    /// - `source` - script source
    pub fn script(source: impl Into<String>) -> Script {
        Script::new(source)
    }
}

/// The `weight` score allows you to multiply the score by the provided weight.
///
/// This can sometimes be desired since boost value set on specific queries gets normalized, while
/// for this score function it does not
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize)]
pub struct Weight {
    weight: f32,
}

impl Weight {
    /// Creates an instance of [Weight](Weight)
    pub fn new(weight: f32) -> Self {
        Self { weight }
    }
}

/// The `random_score` generates scores that are uniformly distributed from `0` up to but not
/// including `1`.
///
/// By default, it uses the internal Lucene doc ids as a source of randomness, which is very
/// efficient but unfortunately not reproducible since documents might be renumbered by merges.
///
/// In case you want scores to be reproducible, it is possible to provide a `seed` and `field`. The
/// final score will then be computed based on this seed, the minimum value of `field` for the
/// considered document and a salt that is computed based on the index name and shard id so that
/// documents that have the same value but are stored in different indexes get different scores.
/// Note that documents that are within the same shard and have the same value for `field` will
/// however get the same score, so it is usually desirable to use a field that has unique values
/// for all documents. A good default choice might be to use the `_seq_no` field, whose only
/// drawback is that scores will change if the document is updated since update operations also
/// update the value of the `_seq_no` field.
#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct RandomScore {
    random_score: RandomScoreInner,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
struct RandomScoreInner {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    seed: Term,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    field: Option<String>,
}

impl RandomScore {
    /// Creates an instance of [RandomScore](RandomScore)
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets seed value
    pub fn seed(mut self, seed: impl Into<Term>) -> Self {
        self.random_score.seed = seed.into();
        self
    }

    /// Sets field value
    pub fn field(mut self, field: impl Into<String>) -> Self {
        self.random_score.field = Some(field.into());
        self
    }
}

/// The `field_value_factor` function allows you to use a field from a document to influence the
/// score.
/// It’s similar to using the `script_score` function, however, it avoids the overhead of scripting.
/// If used on a multi-valued field, only the first value of the field is used in calculations.
///
/// As an example, imagine you have a document indexed with a numeric `my-int` field and wish to
/// influence the score of a document with this field, an example doing so would look like:
/// ```
/// # use elasticsearch_dsl::{FieldValueFactor, FieldValueFactorModifier};
/// # fn main() {
/// # let _ =
/// FieldValueFactor::new("my-int")
///     .factor(1.2)
///     .modifier(FieldValueFactorModifier::Sqrt)
///     .missing(1.0)
/// # ;}
/// ```
/// Which will translate into the following formula for scoring:
/// ```text
/// sqrt(1.2 * doc['my-int'].value)
/// ```
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FieldValueFactor {
    field_value_factor: FieldValueFactorInner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct FieldValueFactorInner {
    field: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    factor: Option<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    modifier: Option<FieldValueFactorModifier>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    missing: Option<f32>,
}

impl FieldValueFactor {
    /// Creates an instance of [FieldValueFactor](FieldValueFactor)
    ///
    /// - `field` - Field to be extracted from the document.
    pub fn new(field: impl Into<String>) -> Self {
        Self {
            field_value_factor: FieldValueFactorInner {
                field: field.into(),
                factor: None,
                modifier: None,
                missing: None,
            },
        }
    }

    /// Factor to multiply the field value with
    pub fn factor(mut self, factor: f32) -> Self {
        self.field_value_factor.factor = Some(factor);
        self
    }

    /// Modifier to apply to the field value
    pub fn modifier(mut self, modifier: FieldValueFactorModifier) -> Self {
        self.field_value_factor.modifier = Some(modifier);
        self
    }

    /// Value used if the document doesn’t have that field. The modifier and factor are still
    /// applied to it as though it were read from the document
    pub fn missing(mut self, missing: f32) -> Self {
        self.field_value_factor.missing = Some(missing);
        self
    }
}

/// Modifier to apply to the field value
///
/// Defaults to [none](FieldValueFactorModifier::None)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FieldValueFactorModifier {
    /// Do not apply any multiplier to the field value
    None,

    /// Take the [common logarithm](https://en.wikipedia.org/wiki/Common_logarithm) of the field
    /// value
    ///
    /// Because this function will return a negative value and cause an error if used on values
    /// between `0` and `1`, it is recommended to use [log1p](FieldValueFactorModifier::Log1P)
    /// instead.
    Log,

    /// Add 1 to the field value and take the common logarithm
    Log1P,

    /// Add 2 to the field value and take the common logarithm
    Log2P,

    /// Take the [natural logarithm](https://en.wikipedia.org/wiki/Natural_logarithm) of the field
    /// value.
    ///
    /// Because this function will return a negative value and cause an error if used on values
    /// between `0` and `1`, it is recommended to use [ln1p](FieldValueFactorModifier::Ln1P)
    /// instead.
    Ln,

    /// Add 1 to the field value and take the natural logarithm
    Ln1P,

    /// Add 2 to the field value and take the natural logarithm
    Ln2P,

    /// Square the field value (multiply it by itself)
    Square,

    /// Take the [square root](https://en.wikipedia.org/wiki/Square_root) of the field value
    Sqrt,

    /// [Reciprocate](https://en.wikipedia.org/wiki/Multiplicative_inverse) the field value, same
    /// as `1/x` where `x` is the field’s value
    Reciprocal,
}

#[doc(hidden)]
pub trait Origin: Debug + PartialEq + Serialize + Clone {
    type Scale: Debug + PartialEq + Serialize + Clone;
    type Offset: Debug + PartialEq + Serialize + Clone;
}

impl Origin for DateTime<Utc> {
    type Scale = Time;
    type Offset = Time;
}

impl Origin for GeoPoint {
    type Scale = Distance;
    type Offset = Distance;
}

macro_rules! impl_origin_for_numbers {
    ($($name:ident ),+) => {
        $(
            impl Origin for $name {
                type Scale = Self;
                type Offset = Self;
            }
        )+
    }
}

impl_origin_for_numbers![i8, i16, i32, i64, u8, u16, u32, u64, f32, f64];

/// Decay functions score a document with a function that decays depending on the distance of a
/// numeric field value of the document from a user given origin. This is similar to a range query,
/// but with smooth edges instead of boxes.
///
/// To use distance scoring on a query that has numerical fields, the user has to define an
/// `origin` and a `scale` for each field. The `origin` is needed to define the “central point”
/// from which the distance is calculated, and the `scale` to define the rate of decay.
#[derive(Debug, Clone, PartialEq)]
pub struct Decay<T: Origin> {
    function: DecayFunction,
    inner: DecayFieldInner<T>,
}
#[derive(Debug, Clone, PartialEq)]
struct DecayFieldInner<T: Origin> {
    field: String,
    inner: DecayInner<T>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct DecayInner<T: Origin> {
    origin: T,

    scale: <T as Origin>::Scale,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    offset: Option<<T as Origin>::Offset>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    decay: Option<f32>,
}

impl<T: Origin> Decay<T> {
    /// Creates an instance of [Decay](Decay)
    ///
    /// - `function` - Decay function variant
    /// - `field` - Field to apply function to
    /// - `origin` - The point of origin used for calculating distance. Must be given as a number
    /// for numeric field, date for date fields and geo point for geo fields. Required for geo and
    /// numeric field. For date fields the default is `now`. Date math (for example now-1h) is
    /// supported for origin.
    /// - `scale` - Required for all types. Defines the distance from origin + offset at which the
    /// computed score will equal `decay` parameter. For geo fields: Can be defined as number+unit
    /// (1km, 12m,…​). Default unit is meters. For date fields: Can to be defined as a number+unit
    /// ("1h", "10d",…​). Default unit is milliseconds. For numeric field: Any number.
    pub fn new(
        function: DecayFunction,
        field: impl Into<String>,
        origin: T,
        scale: <T as Origin>::Scale,
    ) -> Self {
        Self {
            function,
            inner: DecayFieldInner {
                field: field.into(),
                inner: DecayInner {
                    origin,
                    scale,
                    offset: None,
                    decay: None,
                },
            },
        }
    }

    /// If an `offset` is defined, the decay function will only compute the decay function for
    /// documents with a distance greater than the defined `offset`.
    ///
    /// The default is `0`.
    pub fn offset(mut self, offset: <T as Origin>::Offset) -> Self {
        self.inner.inner.offset = Some(offset);
        self
    }

    /// The `decay` parameter defines how documents are scored at the distance given at `scale`. If
    /// no `decay` is defined, documents at the distance `scale` will be scored `0.5`.
    pub fn decay(mut self, decay: f32) -> Self {
        self.inner.inner.decay = Some(decay);
        self
    }
}

impl<T: Origin> Serialize for Decay<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;

        map.serialize_entry(&self.function, &self.inner)?;

        map.end()
    }
}

impl<T: Origin> Serialize for DecayFieldInner<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;

        map.serialize_entry(&self.field, &self.inner)?;

        map.end()
    }
}

/// Decay function variants
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-function-score-query.html#_supported_decay_functions>
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DecayFunction {
    /// Linear decay
    Linear,

    /// Exponential decay
    Exp,

    /// Gauss decay
    Gauss,
}

/// The script_score function allows you to wrap another query and customize the scoring of it
/// optionally with a computation derived from other numeric field values in the doc using a script
/// expression
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Script {
    script_score: ScriptInnerWrapper,
}
#[derive(Debug, Clone, PartialEq, Serialize)]
struct ScriptInnerWrapper {
    script: ScriptInner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct ScriptInner {
    source: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    params: Option<serde_json::Value>,
}

impl Script {
    /// Creates an instance of [Script](Script)
    ///
    /// - `source` - script source
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            script_score: ScriptInnerWrapper {
                script: ScriptInner {
                    source: source.into(),
                    params: None,
                },
            },
        }
    }

    /// Sets params value
    pub fn params(mut self, params: serde_json::Value) -> Self {
        self.script_score.script.params = Some(params);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;

    #[test]
    fn serialization() {
        assert_serialize(
            Decay::new(
                DecayFunction::Gauss,
                "test",
                Utc.ymd(2014, 7, 8).and_hms(9, 1, 0),
                Time::Days(7),
            ),
            json!({
                "gauss": {
                    "test": {
                        "origin": "2014-07-08T09:01:00Z",
                        "scale": "7d",
                    }
                }
            }),
        );

        assert_serialize(
            Decay::new(
                DecayFunction::Exp,
                "test",
                GeoPoint::coordinates(12.0, 13.0),
                Distance::Kilometers(15),
            ),
            json!({
                "exp": {
                    "test": {
                        "origin": [13.0, 12.0],
                        "scale": "15km",
                    }
                }
            }),
        );

        assert_serialize(
            Decay::new(DecayFunction::Linear, "test", 1, 2),
            json!({
                "linear": {
                    "test": {
                        "origin": 1,
                        "scale": 2,
                    }
                }
            }),
        );
    }
}
