//! # Wasm Request
//! Create http request with rustwasm saving boilerplate
//! # WASM Request
//!
//! Create http request with rustwasm saving boilerplate
//!
//! ## How to use
//!
//! ### Include in `Cargo.toml`
//!
//! ```toml
//! wasm_request = '0.1'
//! ```
//!
//! ### Create http request
//!
//! ```rust
//! #[derive(Serialize, Deserialize, Debug, Clone)]
//! struct Eur {
//!     eur: f64,
//! }
//!
//! #[derive(Serialize, Deserialize, Debug, Clone)]
//! struct CurrentPrice {
//!     current_price: Eur,
//! }
//!
//! #[derive(Serialize, Deserialize, Debug, Clone)]
//! struct Price {
//!     market_data: CurrentPrice,
//! }
//!
//! async fn get_token_price() -> Price {
//!     let options = get_options::<Price>(
//!         "https://api.coingecko.com/api/v3/coins/1million-token",
//!         Method::Get,
//!         None,
//!         None,
//!     );
//!
//!     let price_1mt_token: Price = request(options).await.unwrap().into_serde().unwrap();
//!
//!     price_1mt_token
//! }
//! ```
//!
//! ### Post with body
//!
//! ```rust
//! use wasm_request::{get_options, request, Method, DataType};
//!
//! #[derive(Serialize, Deserialize, Debug)]
//! pub struct User {
//!     pub email: String,
//!     pub username: String,
//!     pub password: String,
//! }
//!
//! async fn create_user() {
//!   let user = User {
//!     email: "test@example.com".to_string(),
//!     password: "test43214".to_string(),
//!   }
//!
//!   let options = get_options::<User>(
//!     "https://api.sport-example.com/login",
//!     Method::Post,
//!     None,
//!     Some(DataType::Json(user)),
//!   );
//!
//!   request(options).await.unwrap().into_serde().unwrap();
//! }
//! ```
//!
//! ### Post with form data
//!
//! ```rust
//! use wasm_request::{get_options, request, Method, DataType, get_defualt_headers};
//! use web_sys::FormData;
//!
//! async fn add_form() {
//!   let form = FormData::new().unwrap();
//!   form.append_with_str("name", "Lee").unwrap();
//!   form.append_with_str("email", "test.example@gmail.com").unwrap();
//!
//!   let headers = get_defualt_headers();
//!   headers.delete("Content-Type").unwrap();
//!
//!   let options = get_options::<()>(
//!     "https://api.sport-example.com/user",
//!     Method::Post,
//!     None,
//!     Some(DataType::Form(form)),
//!   );
//!
//!   request(options).await.unwrap().into_serde().unwrap();
//! }
//! ```
//!
//! ### Manage Local Storage
//!
//! ```rust
//! set_storage("food", "ramen");
//! let food = get_storage("food").unwrap();
//! delete_storage("food");
//! ```
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate wasm_bindgen;
extern crate wasm_bindgen_test;
extern crate web_sys;

mod error_handler;

pub use crate::error_handler::RequestError;
use serde::Serialize;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

use web_sys::{window, FormData, Headers, Request, RequestInit, RequestMode, Response, Window};

/// Http methods
#[derive(Debug)]
pub enum Method {
    Get,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTION,
    TRACE,
    CONNECT,
    HEAD,
}

/// DataType of the body, Json or Form
#[derive(Debug)]
pub enum DataType<T> {
    Json(T),
    Form(FormData),
}

/// Request Options
#[derive(Debug)]
pub struct Rq {
    pub url: String,
    pub method: Method,
    pub body: JsValue,
    pub headers: Headers,
}

/// set and return default headers allowing Cors and with Conten-type application/json
pub fn get_defualt_headers() -> Headers {
    let headers = Headers::new().unwrap();
    let token = get_storage("token");

    if token.is_some() {
        if let Some(value) = token {
            headers.append("Authorization", &value).unwrap();
        }
    }

    headers.append("Content-Type", "application/json").unwrap();
    headers.append("Access-Control-Allow-Origin", "*").unwrap();

    headers
}

/// Execute request
///
/// ## Example
/// ```rust
/// let price_1mt_token: Price = request(options).await.unwrap().into_serde().unwrap();
/// ```
pub async fn request(rq: Rq) -> Result<JsValue, RequestError> {
    let mut options = RequestInit::new();

    options.method(&get_method(rq.method));
    options.body(Some(&rq.body));
    options.headers(&rq.headers);
    options.mode(RequestMode::Cors);

    let request_options = Request::new_with_str_and_init(&rq.url, &options)?;

    let wd: Window = window().unwrap();
    let resp_value = JsFuture::from(wd.fetch_with_request(&request_options)).await?;

    let resp: Response = resp_value.dyn_into()?;

    let resp_deserialize = JsFuture::from(resp.json()?).await?;

    if !resp.status().to_string().starts_with('4') && !resp.status().to_string().starts_with('5') {
        Ok(resp_deserialize)
    } else {
        Err(RequestError::from(resp_deserialize))
    }
}

/// Set data by key to local storage
pub fn set_storage(key: &str, value: &str) {
    window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap()
        .set(key, value)
        .unwrap();
}

/// Get data by key from local storage
pub fn get_storage(key: &str) -> Option<String> {
    window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap()
        .get(key)
        .unwrap()
}

/// Delete data by key from local storage
pub fn delete_storage(key: &str) {
    window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap()
        .delete(key)
        .unwrap();
}

/// get options to set in the request
///
/// ## Example
/// ```rust
///     let options = get_options::<()>(
///        "https://api.coingecko.com/api/v3/coins/1million-token",
///        Method::Get,
///        headers,
///        Some(DataType::Form(form)),
///    );
/// ```
pub fn get_options<T>(
    url: &str,
    method: Method,
    headers_option: Option<Headers>,
    data_option: Option<DataType<T>>,
) -> Rq
where
    T: Serialize,
{
    let headers = if let Some(headers) = headers_option {
        headers
    } else {
        get_defualt_headers()
    };

    Rq {
        url: url.to_string(),
        method,
        body: if let Some(data) = data_option {
            match data {
                DataType::Json(json_data) => {
                    let body_string = serde_json::to_string(&json_data).unwrap();
                    JsValue::from_str(&body_string)
                }
                DataType::Form(form_data) => form_data.dyn_into::<JsValue>().unwrap(),
            }
        } else {
            JsValue::null()
        },
        headers,
    }
}

fn get_method(method: Method) -> String {
    match method {
        Method::Get => "GET".to_string(),
        Method::POST => "POST".to_string(),
        Method::PUT => "PUT".to_string(),
        Method::PATCH => "PATCH".to_string(),
        Method::DELETE => "DELETE".to_string(),
        Method::OPTION => "OPTION".to_string(),
        Method::TRACE => "TRACE".to_string(),
        Method::CONNECT => "CONNECT".to_string(),
        Method::HEAD => "HEAD".to_string(),
    }
}

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn should_create_request() {
    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Eur {
        eur: f64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct CurrentPrice {
        current_price: Eur,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Price {
        market_data: CurrentPrice,
    }

    let options = get_options::<()>(
        "https://api.coingecko.com/api/v3/coins/1million-token",
        Method::Get,
        None,
        None,
    );

    let price_1mt_token: Price = request(options).await.unwrap().into_serde().unwrap();

    assert!(!price_1mt_token
        .market_data
        .current_price
        .eur
        .to_string()
        .is_empty());
}
