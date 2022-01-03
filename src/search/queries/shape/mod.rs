//! Like geo_shape Elasticsearch supports the ability to index arbitrary two dimension
//! (non Geospatial) geometries making it possible to map out virtual worlds, sporting venues,
//! theme parks, and CAD diagrams.
//!
//! Elasticsearch supports two types of cartesian data: point fields which support x/y pairs, and
//! shape fields, which support points, lines, circles, polygons, multi-polygons, etc.

mod shape_lookup_query;
mod shape_query;

pub use self::shape_lookup_query::*;
pub use self::shape_query::*;
