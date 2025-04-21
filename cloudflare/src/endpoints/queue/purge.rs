use crate::api_endpoint;
use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::{ApiResult, ApiSuccess};
use serde::{Deserialize, Serialize};

use super::types::Queue;

// INFO: ref: https://developers.cloudflare.com/api/resources/queues/subresources/purge/

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct ConfirmDelete {
    delete_messages_permanently: bool,
}

api_gen!(ConfirmDelete);

// INFO:: Deletes all messages from the Queue.
api_endpoint! (
    POST,
    PurgeQueue => APIResponse > Queue,
    "accounts/{}/queues/{}/purge";
    account_id,
    queue_id;
    params: ConfirmDelete
);
