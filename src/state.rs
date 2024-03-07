use std::sync::Arc;

use tokio::sync::{broadcast, mpsc};

use crate::utils::{arc_mutex, ArcBroadcastSender, ArcMpscSender, ArcMutex};

#[derive(Debug, Clone)]
pub struct AppState {
    pub clipboard_data: ArcMutex<String>,
    // pub short_memory_message: ArcMutex<Vec<RequestMessage>>,
    // pub message_tx: ArcMpscSender<InputMessage>,
    // pub ws_tx: ArcBroadcastSender<MessageSlice>,
    // pub client_n: ArcMutex<u8>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            clipboard_data: arc_mutex(String::new()),
            // short_memory_message: arc_mutex(Vec::new()),
            // message_tx: Arc::new(message_tx),
            // ws_tx: Arc::new(ws_tx),
            // client_n: arc_mutex(0),
        }
    }
}
