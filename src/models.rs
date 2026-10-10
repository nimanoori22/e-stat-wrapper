use serde::{Deserialize, Deserializer, Serialize, de::Error as _};
use serde_json::Value;
use std::collections::BTreeMap;

// e-Stat emits a single object instead of an array for singleton collections.
pub(crate) fn one_or_many<'de, D, T>(deserializer: D) -> std::result::Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: serde::de::DeserializeOwned,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::Null => Ok(Vec::new()),
        Value::Array(values) => values
            .into_iter()
            .map(|v| serde_json::from_value(v).map_err(D::Error::custom))
            .collect(),
        value => Ok(vec![
            serde_json::from_value(value).map_err(D::Error::custom)?,
        ]),
    }
}

pub(crate) fn integer(value: &Value) -> Option<u64> {
    value.as_u64().or_else(|| value.as_str()?.parse().ok())
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Class {
    #[serde(rename = "@code")]
    pub code: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@level", default)]
    pub level: Option<String>,
    #[serde(rename = "@unit", default)]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Dimension {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "CLASS", default, deserialize_with = "one_or_many")]
    pub classes: Vec<Class>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Table {
    #[serde(rename = "@id")]
    pub id: String,
    // These fields can be either a string or an object containing "$".
    #[serde(rename = "TITLE", default)]
    pub title: Value,
    #[serde(rename = "STATISTICS_NAME", default)]
    pub statistics_name: Value,
    #[serde(rename = "UPDATED_DATE", default)]
    pub updated_date: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Metadata {
    pub table: Table,
    pub dimensions: Vec<Dimension>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Observation {
    #[serde(rename = "$")]
    pub raw_value: String,
    #[serde(rename = "@unit", default)]
    pub unit: Option<String>,
    #[serde(rename = "@annotation", default)]
    pub annotation: Option<String>,
    /// Dimension keys retain the wire prefix, e.g. `@area`, `@time`, `@cat01`.
    #[serde(flatten)]
    pub dimensions: BTreeMap<String, Value>,
}

impl Observation {
    pub fn dimension(&self, id: &str) -> Option<&str> {
        self.dimensions.get(&format!("@{id}"))?.as_str()
    }

    /// Suppressed/missing values remain available in `raw_value`; never become zero.
    pub fn numeric_value(&self) -> Option<f64> {
        self.raw_value
            .trim()
            .parse::<f64>()
            .ok()
            .filter(|v| v.is_finite())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct StatsPage {
    pub table: Option<Table>,
    pub observations: Vec<Observation>,
    pub total: u64,
    pub next_position: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TablePage {
    pub tables: Vec<Table>,
    pub total: u64,
    pub next_position: Option<u64>,
}

#[derive(Deserialize)]
pub(crate) struct Classes {
    #[serde(rename = "CLASS_OBJ", default, deserialize_with = "one_or_many")]
    pub dimensions: Vec<Dimension>,
}

#[derive(Deserialize, Default)]
pub(crate) struct Values {
    #[serde(rename = "VALUE", default, deserialize_with = "one_or_many")]
    pub observations: Vec<Observation>,
}
