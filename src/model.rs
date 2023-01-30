use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sale {
    pub id: Option<i64>,
    pub date: Option<std::time::SystemTime>,
    pub sale_kind_id: i32,
    pub frame_brand_id: Option<i32>,
    pub quantity: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Date {
    secs_since_epoch: u128,
    nanos_since_epoch: u128,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameBrand {
    pub id: Option<i32>,
    pub serial: i64,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaleKind {
    id: Option<i32>,
    name: String,
    description: String,
}
