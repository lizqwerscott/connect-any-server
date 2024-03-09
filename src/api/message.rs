use axum::{
    debug_handler,
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::state::AppState;
use crate::{
    datalayer::{clipboard::Clipboard, Device},
    state::ClipboardData,
};

use crate::datalayer::User;
use crate::utils::BDEResult;

use super::{return_base_res, return_bool_res, user};

#[derive(Deserialize)]
pub struct InputDevice {
    name: String,
    #[serde(rename = "type")]
    device_type: String,
}

impl InputDevice {
    fn parse(self) -> BDEResult<Device> {
        let device_type = self.device_type.parse()?;

        Ok(Device::get_device(self.name, device_type)?)
    }
}

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

        let mut clipboard_data = clipboard_datas
            .entry(user.id)
            .or_insert(ClipboardData::new());

        // 超过 100 条剪切板自动清除
        if clipboard_data.data.len() >= 100 {
            clipboard_data.data.remove(0);
        }

        // 根据时间排序
        clipboard_data.data.push(payload.message.clone());

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

        let mut clipboard_data = clipboard_datas
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
