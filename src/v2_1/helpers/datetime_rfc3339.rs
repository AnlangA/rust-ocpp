use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Deserializer, Serializer};

/// Serializes a `DateTime<Utc>` to an RFC3339 string with milliseconds
///
/// This function converts a UTC `DateTime` to a string representation using RFC3339 format
/// with millisecond precision and `true` for `use_z` (appends 'Z' for UTC).
///
/// # Arguments
///
/// * `date` - The `DateTime` to serialize
/// * `serializer` - The serde serializer
///
/// # Returns
///
/// Returns `S::Ok` on successful serialization
///
/// # Errors
///
/// Returns `S::Error` if the serializer fails
pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&date.to_rfc3339_opts(SecondsFormat::Millis, true))
}

/// Deserializes an RFC3339 string to a `DateTime<Utc>`
///
/// This function parses an RFC3339 formatted string and converts it to a UTC `DateTime`.
///
/// # Arguments
///
/// * `deserializer` - The serde deserializer
///
/// # Returns
///
/// Returns `DateTime<Utc>` on successful deserialization
///
/// # Errors
///
/// Returns `D::Error` if:
/// - The input string is not a valid RFC3339 format
/// - The string cannot be parsed as a `DateTime`
pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    DateTime::parse_from_rfc3339(&s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(serde::de::Error::custom)
}
