use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::{broadcast, mpsc};

use crate::datalayer::clipboard::Clipboard;
use crate::datalayer::Device;
use crate::utils::{arc_mutex, ArcBroadcastSender, ArcMpscSender, ArcMutex};

#[derive(Debug, Clone)]
pub struct ClipboardData {
    pub data: Vec<Clipboard>,
    pub devices: Vec<Device>,
}

impl ClipboardData {
    pub fn new() -> Self {
        ClipboardData {
            data: Vec::new(),
            devices: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    // pub clipboard_data: ArcMutex<HashMap<String, Vec<Clipboard>>>,
    pub clipboard_datas: ArcMutex<HashMap<u64, ClipboardData>>,
    // pub short_memory_message: ArcMutex<Vec<RequestMessage>>,
    // pub message_tx: ArcMpscSender<InputMessage>,
    // pub ws_tx: ArcBroadcastSender<MessageSlice>,
    // pub client_n: ArcMutex<u8>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            clipboard_datas: arc_mutex(HashMap::new()),
            // short_memory_message: arc_mutex(Vec::new()),
            // message_tx: Arc::new(message_tx),
            // ws_tx: Arc::new(ws_tx),
            // client_n: arc_mutex(0),
        }
    }
}
