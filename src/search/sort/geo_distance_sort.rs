use super::{SortMode, SortOrder};
use crate::util::{KeyValuePair, ShouldSkip};
use crate::{DistanceUnit, GeoDistanceType, GeoPoint};
use serde::Serialize;

/// Sorts search hits by other field values
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#sort-search-results>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct GeoDistanceSort {
    #[serde(flatten)]
    pair: KeyValuePair<String, Vec<GeoPoint>>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    order: Option<SortOrder>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    unit: Option<DistanceUnit>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    mode: Option<SortMode>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    distance_type: Option<GeoDistanceType>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    ignore_unmapped: Option<bool>,
}

impl GeoDistanceSort {
    /// Creates an instance of [GeoDistanceSort]
    pub fn new<T, U>(field: T, geo_points: U) -> Self
    where
        T: ToString,
        U: IntoIterator,
        U::Item: Into<GeoPoint>,
    {
        Self {
            pair: KeyValuePair::new(
                field.to_string(),
                geo_points.into_iter().map(Into::into).collect(),
            ),
            order: None,
            unit: None,
            mode: None,
            distance_type: None,
            ignore_unmapped: None,
        }
    }

    /// Creates an instance of [GeoDistanceSort] by ascending order
    pub fn ascending<T, U>(field: T, geo_points: U) -> Self
    where
        T: ToString,
        U: IntoIterator,
        U::Item: Into<GeoPoint>,
    {
        Self::new(field, geo_points).order(SortOrder::Asc)
    }

    /// Creates an instance of [GeoDistanceSort] by descending order
    pub fn descending<T, U>(field: T, geo_points: U) -> Self
    where
        T: ToString,
        U: IntoIterator,
        U::Item: Into<GeoPoint>,
    {
        Self::new(field, geo_points).order(SortOrder::Desc)
    }

    /// Explicit order
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#_sort_order>
    pub fn order(mut self, order: SortOrder) -> Self {
        self.order = Some(order);
        self
    }

    /// The unit to use when computing sort values
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#geo-sorting>
    pub fn unit(mut self, unit: DistanceUnit) -> Self {
        self.unit = Some(unit);
        self
    }

    /// Sort mode for numeric fields
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#_sort_mode_option>
    pub fn mode(mut self, mode: SortMode) -> Self {
        self.mode = Some(mode);
        self
    }

    /// How to compute the distance. Can either be arc (default), or plane (faster, but inaccurate on long distances and close to the poles).
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#_sort_mode_option>
    pub fn distance_type(mut self, distance_type: GeoDistanceType) -> Self {
        self.distance_type = Some(distance_type);
        self
    }

    /// Indicates if the unmapped field should be treated as a missing value. Setting it to `true`
    /// is equivalent to specifying an `unmapped_type` in the field sort. The default is `false`
    /// (unmapped field cause the search to fail).
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#_sort_mode_option>
    pub fn ignore_unmapped(mut self, ignore_unmapped: bool) -> Self {
        self.ignore_unmapped = Some(ignore_unmapped);
        self
    }
}

impl IntoIterator for GeoDistanceSort {
    type Item = Self;

    type IntoIter = std::option::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        Some(self).into_iter()
    }
}

serialize_query!("_geo_distance": GeoDistanceSort);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::assert_serialize;

    #[test]
    fn serialization() {
        assert_serialize(
            GeoDistanceSort::new("test", GeoPoint::coordinates(1.2, 3.3)),
            json!({
                "_geo_distance": {
                    "test": [ [3.3, 1.2] ]
                }
            }),
        );

        assert_serialize(
            GeoDistanceSort::ascending("test", GeoPoint::coordinates(1.2, 3.3)),
            json!({
                "_geo_distance": {
                    "test": [ [3.3, 1.2] ],
                    "order": "asc",
                }
            }),
        );

        assert_serialize(
            GeoDistanceSort::descending("test", GeoPoint::coordinates(1.2, 3.3))
                .order(SortOrder::Asc)
                .unit(DistanceUnit::Inches)
                .mode(SortMode::Max)
                .distance_type(GeoDistanceType::Arc)
                .ignore_unmapped(true),
            json!({
                "_geo_distance": {
                    "test": [ [3.3, 1.2] ],
                    "unit": "in",
                    "order": "asc",
                    "mode": "max",
                    "distance_type": "arc",
                    "ignore_unmapped": true,
                }
            }),
        );
    }
}
