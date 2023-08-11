use std::fmt::Formatter;
use chrono::{DateTime, TimeZone, Utc};
use rusqlite::ToSql;
use rusqlite::types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};

/// A wrapper around a chrono datetime, restricted to a second for resolution.
/// Serializes to an integer being the number of seconds from the last unix epoch.
#[derive(Debug, PartialEq, Clone)]
pub struct MyDateTime(pub DateTime<Utc>);

impl From<i64> for MyDateTime {
    fn from(value: i64) -> Self {
        MyDateTime (
            Utc.timestamp_opt(value, 0).unwrap()
        )
    }
}

impl From<DateTime<Utc>> for MyDateTime {
    /// Note: may result in precision loss.
    fn from(value: DateTime<Utc>) -> Self {
        MyDateTime::from(value.timestamp())
    }
}

impl FromSql for MyDateTime {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let timestamp_seconds = i64::column_result(value)?;
        Ok(MyDateTime::from(timestamp_seconds))
    }
}

impl ToSql for MyDateTime {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.0.timestamp()))
    }
}

impl Serialize for MyDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_i64(self.0.timestamp())
    }
}

impl<'de> Deserialize<'de> for MyDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_i64(MyDateTimeVisitor)
    }
}

struct MyDateTimeVisitor;
impl<'de> Visitor<'de> for MyDateTimeVisitor {
    type Value = MyDateTime;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "a 64 bit signed integer")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> where E: Error {
        Ok(MyDateTime::from(v))
    }
}