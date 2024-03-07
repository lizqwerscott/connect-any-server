pub mod api;
mod datalayer;
mod state;
mod utils;
mod websocket;

use state::AppState;

pub async fn init() -> AppState {
    AppState::new()
}
