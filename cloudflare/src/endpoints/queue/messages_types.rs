use crate::framework::response::ApiResult;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct ActionMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    acks: Option<Vec<AcksRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retries: Option<Vec<RetriesRequest>>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct AcksRequest {
    // An ID that represents an "in-flight" message that has been pulled from a Queue. You must hold on to this ID and use it to acknowledge this message.
    lease_id: String,
}
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct RetriesRequest {
    // The number of seconds to delay before making the message available for another attempt.
    #[serde(skip_serializing_if = "Option::is_none")]
    delay_seconds: Option<i64>,
    // An ID that represents an "in-flight" message that has been pulled from a Queue. You must hold on to this ID and use it to acknowledge this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    lease_id: Option<String>,
}

api_resp!(AckQueue {
    ack_count: i64,
    retry_count: i64,
    warnings: Vec<String>,
});

api_gen!(AckQueueResponse);

api_resp!(PullQueue {
    id: String,
    attempts: i64,
    body: String,
    lease_id: String,
});

api_gen!(PullQueueResponse);
