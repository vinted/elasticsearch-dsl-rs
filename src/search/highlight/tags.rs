use serde::ser::{Serialize, SerializeStruct, Serializer};

/// Set to `styled` to use the built-in tag schema or use custom tags
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tags {
    /// Defines the following `pre_tags` and defines `post_tags` as `</em>`.
    ///
    /// ```html
    /// <em class="hlt1">, <em class="hlt2">, <em class="hlt3">,
    /// <em class="hlt4">, <em class="hlt5">, <em class="hlt6">,
    /// <em class="hlt7">, <em class="hlt8">, <em class="hlt9">,
    /// <em class="hlt10">
    /// ```
    Styled,

    /// Custom pre/post tags
    Custom(PrePostTags),
}

/// Contains `pre_tags` and `post_tags` highlighting values
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct PrePostTags {
    pre_tags: Vec<String>,
    post_tags: Vec<String>,
}

impl PrePostTags {
    /// Creates a new instance of PrePostTags
    pub fn new<I>(pre_tags: I, post_tags: I) -> Self
    where
        I: IntoIterator,
        I::Item: ToString,
    {
        Self {
            pre_tags: pre_tags.into_iter().map(|x| x.to_string()).collect(),
            post_tags: post_tags.into_iter().map(|x| x.to_string()).collect(),
        }
    }
}
impl<T, const N: usize> From<([T; N], [T; N])> for Tags
where
    T: ToString,
{
    fn from(values: ([T; N], [T; N])) -> Self {
        Self::Custom(PrePostTags {
            pre_tags: values.0.iter().map(ToString::to_string).collect(),
            post_tags: values.1.iter().map(ToString::to_string).collect(),
        })
    }
}

impl<T, const N: usize> From<([T; N], [T; N])> for PrePostTags
where
    T: ToString,
{
    fn from(values: ([T; N], [T; N])) -> Self {
        Self {
            pre_tags: values.0.iter().map(ToString::to_string).collect(),
            post_tags: values.1.iter().map(ToString::to_string).collect(),
        }
    }
}

impl<T> From<(Vec<T>, Vec<T>)> for Tags
where
    T: ToString,
{
    fn from(values: (Vec<T>, Vec<T>)) -> Self {
        Tags::Custom(PrePostTags::new(values.0, values.1))
    }
}

impl<T> From<(Vec<T>, Vec<T>)> for PrePostTags
where
    T: ToString,
{
    fn from(values: (Vec<T>, Vec<T>)) -> Self {
        Self::new(values.0, values.1)
    }
}

impl<'a, T> From<(&'a [T], &'a [T])> for Tags
where
    T: ToString,
{
    fn from(values: (&'a [T], &'a [T])) -> Self {
        Tags::Custom(PrePostTags {
            pre_tags: values.0.iter().map(ToString::to_string).collect(),
            post_tags: values.1.iter().map(ToString::to_string).collect(),
        })
    }
}

impl<'a, T> From<(&'a [T], &'a [T])> for PrePostTags
where
    T: ToString,
{
    fn from(values: (&'a [T], &'a [T])) -> Self {
        Self {
            pre_tags: values.0.iter().map(ToString::to_string).collect(),
            post_tags: values.1.iter().map(ToString::to_string).collect(),
        }
    }
}

impl Serialize for Tags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Styled => {
                let mut map = serializer.serialize_struct("TagsStyled", 1)?;
                map.serialize_field("tags_schema", "styled")?;
                map.end()
            }
            Self::Custom(ref tags) => Serialize::serialize(tags, serializer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_serialization! {
        styled(
            Tags::Styled,
            json!({ "tags_schema": "styled" })
        );

        custom(
            Tags::Custom((["<h1>"], ["</h1>"]).into()),
            json!({
                "pre_tags": ["<h1>"],
                "post_tags": ["</h1>"]
            })
        );
    }
}
