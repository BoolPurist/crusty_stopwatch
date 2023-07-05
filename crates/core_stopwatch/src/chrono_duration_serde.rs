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
    d.deserialize_str(DurationDeserilizer)
}

struct DurationDeserilizer;
impl<'de> de::Visitor<'de> for DurationDeserilizer {
    type Value = Duration;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a datetime string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Duration, E>
    where
        E: de::Error,
    {
        let number: i64 = value
            .parse()
            .map_err(|_| E::custom(format!("{} is a valid number for seconds", value)))?;
        Ok(chrono::Duration::seconds(number))
    }
}
