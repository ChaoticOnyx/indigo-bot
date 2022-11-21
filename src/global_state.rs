use crate::prelude::*;

#[async_trait]
pub trait GlobalState {
    /// Returns **copy** of the state.
    async fn get_state() -> Self;
    async fn set_state(value: Self);
}
