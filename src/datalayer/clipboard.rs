use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ClipboardDataType {
    Text,
    Image,
    None,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Clipboard {
    pub data: String,
    #[serde(rename = "type")]
    pub clipboard_type: ClipboardDataType,
    pub date: u128,
}

impl Clipboard {
    pub fn new(data: String, clipboard_type: ClipboardDataType) -> Self {
        let date = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        Clipboard {
            data,
            clipboard_type,
            date,
        }
    }

    pub fn empty() -> Self {
        Clipboard {
            data: String::new(),
            clipboard_type: ClipboardDataType::None,
            date: 0,
        }
    }
}

impl fmt::Display for Clipboard {
    // 这个 trait 要求 `fmt` 使用与下面的函数完全一致的函数签名
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let duration = UNIX_EPOCH + std::time::Duration::from_millis(self.date as u64);
        let datetime = DateTime::<Local>::from(duration);
        let formatted_datetime = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        let output = match self.clipboard_type {
            ClipboardDataType::Text => self.data.clone(),
            ClipboardDataType::Image => String::from("Image"),
            ClipboardDataType::None => String::from("None"),
        };
        write!(f, "Clipboard[{}]: {}", formatted_datetime, output)
    }
}
