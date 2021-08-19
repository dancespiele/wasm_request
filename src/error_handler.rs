use std::error::Error;
use std::fmt::{Debug, Display, Error as ErrorFmt, Formatter};
use wasm_bindgen::JsValue;

/// Error Handler
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RequestError {
    pub status: i16,
    pub message: String,
}

impl Display for RequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), ErrorFmt> {
        Debug::fmt(&self, f)
    }
}

impl Error for RequestError {}

impl From<JsValue> for RequestError {
    fn from(value: JsValue) -> Self {
        let request_error: RequestError = value.into_serde().unwrap();
        request_error
    }
}
