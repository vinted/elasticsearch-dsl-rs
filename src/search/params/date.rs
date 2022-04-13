use chrono::{DateTime, Utc};
use std::time::SystemTime;

/// [`DateTime<Utc>`] type alias
pub type ChronoTime = DateTime<Utc>;

/// Time variants to serialize
#[derive(Clone, Copy, Serialize)]
#[serde(untagged)]
pub enum Date {
    /// System time
    System(SystemTime),

    /// Chrono time
    Chrono(ChronoTime),
}

impl std::fmt::Debug for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::System(value) => value.fmt(f),
            Self::Chrono(value) => value.fmt(f),
        }
    }
}

impl From<SystemTime> for Date {
    fn from(value: SystemTime) -> Self {
        Self::System(value)
    }
}

impl From<ChronoTime> for Date {
    fn from(value: ChronoTime) -> Self {
        Self::Chrono(value)
    }
}

impl From<&SystemTime> for Date {
    fn from(value: &SystemTime) -> Self {
        Self::System(*value)
    }
}

impl From<&ChronoTime> for Date {
    fn from(value: &ChronoTime) -> Self {
        Self::Chrono(*value)
    }
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Date::System(s), Date::System(o)) => s.eq(o),
            (Date::System(s), Date::Chrono(o)) => ChronoTime::from(*s).eq(o),
            (Date::Chrono(s), Date::System(o)) => s.eq(&ChronoTime::from(*o)),
            (Date::Chrono(s), Date::Chrono(o)) => s.eq(o),
        }
    }
}

impl PartialEq<SystemTime> for Date {
    fn eq(&self, other: &SystemTime) -> bool {
        match self {
            Self::System(s) => s.eq(other),
            Self::Chrono(s) => s.eq(&ChronoTime::from(*other)),
        }
    }
}

impl PartialEq<ChronoTime> for Date {
    fn eq(&self, other: &ChronoTime) -> bool {
        match self {
            Self::Chrono(s) => s.eq(other),
            Self::System(s) => ChronoTime::from(*s).eq(other),
        }
    }
}

impl Eq for Date {}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd<SystemTime> for Date {
    fn partial_cmp(&self, other: &SystemTime) -> Option<std::cmp::Ordering> {
        match self {
            Date::System(s) => s.partial_cmp(other),
            Date::Chrono(s) => s.partial_cmp(&ChronoTime::from(*other)),
        }
    }
}

impl PartialOrd<ChronoTime> for Date {
    fn partial_cmp(&self, other: &ChronoTime) -> Option<std::cmp::Ordering> {
        match self {
            Self::Chrono(s) => s.partial_cmp(other),
            Self::System(s) => ChronoTime::from(*s).partial_cmp(other),
        }
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Date::System(s), Date::System(o)) => s.cmp(o),
            (Date::System(s), Date::Chrono(o)) => ChronoTime::from(*s).cmp(o),
            (Date::Chrono(s), Date::System(o)) => s.cmp(&ChronoTime::from(*o)),
            (Date::Chrono(s), Date::Chrono(o)) => s.cmp(o),
        }
    }
}
