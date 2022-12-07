pub use crate::api::Api;
pub use crate::async_closure;
pub use crate::state::{
    DiscordSession, GlobalStateClone, GlobalStateLock, GlobalStateSet, Settings,
};
pub use futures_util::FutureExt;
pub use serenity::async_trait;
pub use tracing::{debug, error, info, instrument, warn};

pub mod discord {
    pub use serenity::model::*;
    pub use serenity::prelude::*;
}

pub mod github {
    pub use octocrab::models::*;
}

pub mod byond {
    pub use crate::api::models::ByondCkey as UserId;
}

pub mod ss14 {
    pub use crate::api::models::SS14Guid as UserId;
}
