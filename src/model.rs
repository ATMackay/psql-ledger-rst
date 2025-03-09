use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize}; //
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, Serialize, Debug)]
pub struct AccountParams {
    pub id: Option<i64>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub balance: Option<i64>,
}

#[derive(Deserialize, PostgresMapper, Serialize, Debug)]
#[pg_mapper(table = "accounts")]
pub struct Account {
    pub id: Option<i64>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub balance: Option<i64>,
    #[serde(
        serialize_with = "serialize_datetime",
        deserialize_with = "deserialize_datetime",
        skip_serializing_if = "Option::is_none", // Skip serializing if None
        default // Use default for deserialization, which for Option<T> is None
    )]
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TransactionParams {
    pub id: Option<i64>,
    pub from_account: Option<i64>,
    pub to_account: Option<i64>,
    pub amount: Option<i64>,
}

#[derive(Deserialize, PostgresMapper, Serialize, Debug)]
#[pg_mapper(table = "transactions")]
pub struct Transaction {
    pub id: Option<i64>,
    pub from_account: Option<i64>,
    pub to_account: Option<i64>,
    pub amount: Option<i64>,
    #[serde(
        serialize_with = "serialize_datetime",
        deserialize_with = "deserialize_datetime",
        skip_serializing_if = "Option::is_none", // Skip serializing if None
        default // Use default for deserialization, which for Option<T> is None
    )]
    pub created_at: Option<DateTime<Utc>>,
}

// status represents the default JSON
// response format (also used to encode error messages)
#[derive(Deserialize, Serialize, Debug)]
pub struct Status {
    pub service: String,
    pub version: String,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Health {
    pub service: String,
    pub version: String,
    pub failures: Vec<String>,
}

// Custom serialization function for DateTime<Utc>
fn serialize_datetime<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if let Some(ref date) = *date {
        let s = date.to_rfc3339();
        serializer.serialize_some(&s)
    } else {
        serializer.serialize_none()
    }
}

fn deserialize_datetime<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = Option::<String>::deserialize(deserializer)?;
    if let Some(s) = s {
        s.parse::<DateTime<Utc>>()
            .map(Some)
            .map_err(serde::de::Error::custom)
    } else {
        Ok(None)
    }
}
