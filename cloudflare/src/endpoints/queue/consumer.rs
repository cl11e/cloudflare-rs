use crate::api_endpoint;
use crate::endpoints::shared_types::APIResponse;
use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::{ApiResult, ApiSuccess};
use serde::{Deserialize, Serialize};

use super::consumer_types::Consumer;

// INFO: ref. https://developers.cloudflare.com/api/resources/queues/subresources/consumers/

// INFO: Creates a new consumer for a Queue
api_endpoint!(
    POST,
    CreateQueueConsumer => APIResponse > Consumer,
    "accounts/{}/queues/{}/consumers";
    account_id,
    queue_id;
    params: Consumer
);

// INFO: Deletes the consumer for a queue.
api_endpoint!(
    DELETE,
    DeleteQueueConsumer => APIResponse,
    "accounts/{}/queues/{}/consumers/{}";
    account_id,
    queue_id,
    consumer_id
);

// INFO: Returns the consumers for a Queue
api_endpoint!(
    GET,
    ListQueueConsumer => APIResponse > Vec<Consumer> ,
    "accounts/{}/queues/{}/consumers";
    account_id,
    queue_id
);

// INFO: Updates the consumer for a queue, or creates one if it does not exist.
api_endpoint!(
    PUT,
    UpdateQueueConsumer => APIResponse > Consumer,
    "accounts/{}/queues/{}/consumers/{}";
    account_id,
    queue_id,
    consumer_id;
    params: Consumer
);
