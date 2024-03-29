pub mod api;
mod bark;
mod datalayer;
mod state;
mod utils;
pub mod websocket;

use state::AppState;

pub async fn init() -> AppState {
    AppState::new()
}
