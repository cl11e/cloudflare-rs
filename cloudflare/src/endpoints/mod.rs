/*!
Implementations of the Endpoint trait for individual Cloudflare API endpoints, e.g. DNS or Workers.
If you want to add a new Cloudflare API to this crate, simply add a new submodule of this `endpoints`
module.
 */
pub mod account;
pub mod ai;
pub mod argo_tunnel;
pub mod cfd_tunnel;
pub mod dns;
pub mod load_balancing;
#[macro_use]
mod api_endpoint;
pub mod queue;
pub mod r2;
pub mod workers;
pub mod workerskv;
pub mod zones;
