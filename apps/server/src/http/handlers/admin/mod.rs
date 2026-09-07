//! Admin API handlers.
//!
//! [CB §27] — Multi-User Authentication & Team Management

pub mod global_knowledge;
pub mod global_providers;
pub mod invitations;
pub mod system_config;
pub mod users;

pub use global_knowledge::{list_global_knowledge, save_global_knowledge};
pub use global_providers::{list_global_providers, save_global_provider};
pub use invitations::create_invitation;
pub use users::{create_user, delete_user, list_users};