/// Gap policies are a mechanism to inform the pipeline aggregation about the desired behavior when
/// "gappy" or missing data is encountered. All pipeline aggregations accept the `gap_policy`
/// parameter.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GapPolicy {
    /// This option treats missing data as if the bucket does not exist. It will skip the bucket
    /// and continue calculating using the next available value.
    Skip,

    /// This option will replace missing values with a zero (0) and pipeline aggregation
    /// computation will proceed as normal.
    InsertZeros,

    /// This option is similar to skip, except if the metric provides a non-null, non-NaN value
    /// this value is used, otherwise the empty bucket is skipped.
    KeepValues,
}
