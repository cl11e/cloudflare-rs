use crate::framework::response::ResponseInfo;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct APIResponse {
    errors: Option<Vec<ResponseInfo>>,
    messages: Option<Vec<String>>,
    success: Option<bool>,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}
