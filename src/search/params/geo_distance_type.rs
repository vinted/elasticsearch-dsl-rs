/// How to compute the distance
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeoDistanceType {
    /// Faster, but inaccurate on long distances and close to the poles
    Plane,

    /// Default
    Arc,
}
