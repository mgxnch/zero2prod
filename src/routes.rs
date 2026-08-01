mod health_check;
mod subscriptions;

// Re-exported so that consumers of routes module don't need to reach too deep
// Instead of `routes::subscriptions::subscribe`, consumers can call `routes::subscribe`.
pub use health_check::*;
pub use subscriptions::*;
