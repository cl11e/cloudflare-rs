use crate::api_endpoint;
use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::{ApiResult, ApiSuccess};
use serde::{Deserialize, Serialize};
pub mod consumer;
pub mod consumer_types;
pub mod messages;
pub mod messages_types;
pub mod purge;
pub mod types;

use types::*;

use super::shared_types::APIResponse;

api_gen!(APIResponse);
// INFO: ref: https://developers.cloudflare.com/api/resources/queues/

// INFO: Create a new queue
api_endpoint!(
    POST,
    CreateQueue => APIResponse > Queue,
    "accounts/{}/queues";
    account_id;
    params: Queue// Explicit params marker
);

// INFO: Deletes a queue
api_endpoint!(
    DELETE,
    DeleteQueue => APIResponse,
    "accounts/{}/queues/{}";
    account_id,
    queue_id
);

api_gen!(Queue);

// INFO: Updates a Queue.
api_endpoint!(
    PATCH,
    UpdateQueue => Queue > Queue,
    "accounts/{}/queues/{}";
    account_id,
    queue_id;
    params: Queue
);

// INFO: Get details about a specific queue.
api_endpoint!(
    GET,
    GetQueue => APIResponse > Queue,
    "accounts/{}/queues/{}";
    account_id,
    queue_id
);

// INFO: Returns the queues owned by an account.
api_endpoint!(
    GET,
    ListQueue => APIResponse > Vec<Queue>,
    "accounts/{}/queues";
    account_id
);
