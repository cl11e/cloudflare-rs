use crate::api_endpoint;
use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::ApiSuccess;

use super::messages_types::{AckQueueResponse, ActionMessage, PullQueueResponse};

// INFO: ref. https://developers.cloudflare.com/api/resources/queues/subresources/messages/

// INFO:  Acknowledge + Retry messages from a Queue
api_endpoint!(
    POST,
    AckRetry => AckQueueResponse,
    "accounts/{}/queues/{}/messages/ack";
    account_id,
    queue_id;
    params: ActionMessage
);

// INFO: Pull a batch of messages from a Queue
api_endpoint!(
    POST,
    PullQueue => PullQueueResponse,
    "/accounts/{}/queues/{}/messages/pull";
    account_id,
    queue_id
);
