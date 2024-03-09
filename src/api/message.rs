use axum::{debug_handler, extract::State, response::IntoResponse, Json};
use serde::Deserialize;

use crate::state::AppState;
use crate::{
    datalayer::{clipboard::Clipboard, Device, InputDevice},
    state::ClipboardData,
};

use crate::datalayer::User;

use super::{return_base_res, return_bool_res};

#[derive(Deserialize)]
pub struct InputAddMessage {
    device: InputDevice,
    message: Clipboard,
}

#[debug_handler]
pub async fn add_message(
    State(state): State<AppState>,
    Json(payload): Json<InputAddMessage>,
) -> impl IntoResponse {
    let handler = || async {
        let now_device = payload.device.parse()?;

        let user = User::find_user_from_device(&now_device)?;

        let mut clipboard_datas = state.clipboard_datas.lock().await;

        let clipboard_data = clipboard_datas
            .entry(user.id)
            .or_insert(ClipboardData::new());

        // 超过 100 条剪切板自动清除
        if clipboard_data.data.len() >= 100 {
            clipboard_data.data.remove(0);
        }

        // TODO: 根据时间排序
        clipboard_data.data.push(payload.message.clone());

        // websocket
        let ws_tx = clipboard_data.ws_tx.clone();
        if let Err(err) = ws_tx.send(payload.message.clone()) {
            tracing::error!("send websocket error: {}", err);
        }

        // TODO: 如果 websocket 发送了, 就不添加进入需要更新的设备列表
        let mut need_update_devices: Vec<Device> = Vec::new();

        for device in user.devices.into_iter() {
            if device != now_device {
                need_update_devices.push(device);
            }
        }

        clipboard_data.devices = need_update_devices;

        tracing::info!(
            "device ({}) add message: {}",
            now_device.name,
            payload.message
        );

        Ok(())
    };

    Json(return_bool_res(handler().await))
}

#[derive(Deserialize)]
pub struct InputMessageUpdateBase {
    device: InputDevice,
}

#[debug_handler]
pub async fn message_update_base(
    State(state): State<AppState>,
    Json(payload): Json<InputMessageUpdateBase>,
) -> impl IntoResponse {
    let handler = || async {
        let now_device = payload.device.parse()?;

        let user = User::find_user_from_device(&now_device)?;

        let mut clipboard_datas = state.clipboard_datas.lock().await;

        let clipboard_data = clipboard_datas
            .entry(user.id)
            .or_insert(ClipboardData::new());

        if let Some(data) = clipboard_data.data.last() {
            if let Some(index) = clipboard_data.devices.iter().position(|x| x == &now_device) {
                clipboard_data.devices.remove(index);

                return Ok(data.clone());
            }
        }

        Ok(Clipboard::empty())
    };

    Json(return_base_res(handler().await))
}
