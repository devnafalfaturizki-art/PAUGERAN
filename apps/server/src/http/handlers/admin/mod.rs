//! Admin API handlers.
//! 
//! [CB §27] — Multi-User Authentication & Team Management

pub mod global_knowledge;
pub mod global_providers;
pub mod invitations;
pub mod system_config;
pub mod users;

pub use users::{create_user, delete_user, list_users};
