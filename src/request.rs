use crate::model;
use serde::{Deserialize, Serialize};
const URL: &str = "http://127.0.0.1:8000";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RequestError {
    FetchError(String),
    SerializationError(String),
}

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[ERROR]: {self:?}"))
    }
}

impl std::error::Error for RequestError {}

impl From<reqwasm::Error> for RequestError {
    fn from(value: reqwasm::Error) -> Self {
        Self::FetchError(value.to_string())
    }
}

impl From<leptos::SerializationError> for RequestError {
    fn from(_: leptos::SerializationError) -> Self {
        todo!()
    }
}

// pub async fn fetch_frame_brand(id: u32) -> Result<model::FrameBrand, RequestError> {
//     reqwasm::http::Request::get(&format!("{URL}/frame_brands/{id}"))
//         .send()
//         .await
//         .map_err(RequestError::from)?
//         .json()
//         .await
//         .map_err(RequestError::from)
// }

pub async fn fetch_frame_brands(_: ()) -> Result<Vec<model::FrameBrand>, RequestError> {
    reqwasm::http::Request::get(&format!("{URL}/frame_brands"))
        .send()
        .await
        .map_err(RequestError::from)?
        .json()
        .await
        .map_err(RequestError::from)
}

// pub async fn fetch_sale_kind(id: u32) -> Result<model::SaleKind, RequestError> {
//     reqwasm::http::Request::get(&format!("{URL}/sale_kinds/{id}"))
//         .send()
//         .await
//         .map_err(RequestError::from)?
//         .json()
//         .await
//         .map_err(RequestError::from)
// }

pub async fn fetch_sale_kinds(_: ()) -> Result<Vec<model::SaleKind>, RequestError> {
    reqwasm::http::Request::get(&format!("{URL}/sale_kinds"))
        .send()
        .await
        .map_err(RequestError::from)?
        .json()
        .await
        .map_err(RequestError::from)
}

pub async fn fetch_sales(_: ()) -> Result<Vec<model::Sale>, RequestError> {
    reqwasm::http::Request::get(&format!("{URL}/sales"))
        .send()
        .await
        .map_err(RequestError::from)?
        .json()
        .await
        .map_err(RequestError::from)
}

// pub async fn fetch_sale(id: Option<u32>) -> Result<model::Sale, RequestError> {
//     let sale_id = id
//         .ok_or("Unnecesary Fetch for fetch sale function")
//         .unwrap_throw();
//     reqwasm::http::Request::get(&format!("{URL}/sales/{sale_id}"))
//         .send()
//         .await
//         .map_err(RequestError::from)?
//         .json()
//         .await
//         .map_err(RequestError::from)
// }

pub async fn post_sale(sale: Option<model::PostSale>) -> Result<model::Sale, RequestError> {
    let new_sale = sale.ok_or(RequestError::FetchError("No Sale to Upload!".into()))?;
    reqwasm::http::Request::post(&format!("{URL}/sales"))
        .body(
            serde_json::to_string(&new_sale)
                .map_err(|e| RequestError::SerializationError(e.to_string()))?,
        )
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(RequestError::from)?
        .json()
        .await
        .map_err(RequestError::from)
}

pub async fn delete_sale(id: Option<u32>) -> Result<u32, RequestError> {
    let id = id.ok_or(RequestError::FetchError("No ID to delete.".into()))?;
    reqwasm::http::Request::delete(&format!("{URL}/sales/{id}",))
        .send()
        .await
        .map_err(RequestError::from)?
        .json()
        .await
        .map_err(RequestError::from)
}

pub async fn update_sale(sale: Option<model::Sale>) -> Result<model::Sale, RequestError> {
    let new_sale = sale.ok_or(RequestError::FetchError("No Sale to Upload!".into()))?;
    reqwasm::http::Request::put(&format!("{URL}/sales/{}", new_sale.id.unwrap()))
        .body(
            serde_json::to_string(&new_sale)
                .map_err(|e| RequestError::SerializationError(e.to_string()))?,
        )
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(RequestError::from)?
        .json()
        .await
        .map_err(RequestError::from)
}
