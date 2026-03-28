use serde::{Deserialize, Deserializer};

pub fn de_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.to_lowercase().as_str() {
        "1.0" | "true" | "1" | "yes" => Ok(true),
        "" | "0.0" | "false" | "0" | "no" => Ok(false),
        _ => Err(serde::de::Error::custom("invalid bool")),
    }
}
