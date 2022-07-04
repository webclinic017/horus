use chrono::{DateTime, Utc};

pub struct TimeSeriesElement<DATATYPE> {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub datum: DATATYPE
}