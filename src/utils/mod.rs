use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, Mutex};

pub mod database;

pub type ArcMutex<T> = Arc<Mutex<T>>;
pub type ArcMpscSender<T> = Arc<mpsc::Sender<T>>;
pub type ArcBroadcastSender<T> = Arc<broadcast::Sender<T>>;

pub type BDError = Box<dyn std::error::Error>;
pub type BDEResult<T> = Result<T, BDError>;

#[derive(Debug, Clone)]
pub struct AiError {
    err: String,
}

impl AiError {
    pub fn new(err: &str) -> AiError {
        AiError {
            err: err.to_string(),
        }
    }
}

impl std::fmt::Display for AiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.err)
    }
}

impl std::error::Error for AiError {
    fn description(&self) -> &str {
        &self.err
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        // 泛型错误。没有记录其内部原因。
        None
    }
}

pub fn ba_error(error: &str) -> Box<dyn std::error::Error> {
    Box::new(AiError::new(error))
}

pub fn arc_mutex<T>(data: T) -> ArcMutex<T> {
    Arc::new(Mutex::new(data))
}

pub fn log_error(prompt: &str, res: BDEResult<()>) {
    if let Err(err) = res {
        tracing::error!("{} error: {}", prompt, err.to_string());
    }
}
