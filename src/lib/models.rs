//use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize}; // Serializer
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "accounts")]
pub struct Account {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub balance: i64,
    //#[serde(serialize_with = "serialize_datetime")]
    //pub created_at: DateTime<Utc>,
}
#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "transactions")]
pub struct Transaction {
    pub id: i64,
    pub from_account: i64,
    pub to_account: i64,
    pub amount: i64,
    //#[serde(serialize_with = "serialize_datetime")]
    //pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
pub struct Status {
    pub service: String,
    pub version: String,
    pub message: String,
}

#[derive(Deserialize, Serialize)]
pub struct Health {
    pub service: String,
    pub version: String,
    pub message: String,
    pub failures: Vec<String>,
}

// Custom serialization function for DateTime<Utc> - TODO
//fn serialize_datetime<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
//where
//    S: Serializer,
//{
//    let s = date.to_rfc3339();
//    serializer.serialize_str(&s)
//}
