#[allow(clippy::module_inception)]
mod rights;
mod scoped_rights;
mod service_rights;
mod token_rights;
mod user_rights;

pub use rights::Rights;
pub use scoped_rights::RightsScope;
pub use service_rights::{ScopedServiceRights, ServiceRights};
pub use token_rights::TokenRights;
pub use user_rights::UserRights;
