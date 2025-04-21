use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum Consumer {
    Worker(MqWorkerConsumer),
    HttpPull(MqHttpConsumer),
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct MqWorkerConsumer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_id: Option<String>, // Optional (maxLength: 32)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_queue: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<String>, // Optional (maxLength: 32)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<WorkerSettings>,

    #[serde(rename = "type")]
    pub type_: Option<String>, // "worker"
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct WorkerSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_wait_time_ms: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_delay: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct MqHttpConsumer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_id: Option<String>, // Optional (maxLength: 32)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_queue: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<String>, // Optional (maxLength: 32)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<HttpSettings>,

    #[serde(rename = "type")]
    pub type_: Option<String>, // "http_pull"
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct HttpSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_delay: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility_timeout_ms: Option<u64>,
}
