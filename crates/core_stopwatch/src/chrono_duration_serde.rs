use std::fmt;

use chrono::Duration;
use serde::{de, ser};

pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    serializer.serialize_i64(duration.num_seconds())
}

pub fn deserialize<'de, D>(d: D) -> Result<Duration, D::Error>
where
    D: de::Deserializer<'de>,
{
    d.deserialize_i64(DurationDeserilizer)
}

struct DurationDeserilizer;
impl<'de> de::Visitor<'de> for DurationDeserilizer {
    type Value = Duration;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Not second number")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Duration, E>
    where
        E: de::Error,
    {
        Ok(chrono::Duration::seconds(value))
    }
}
