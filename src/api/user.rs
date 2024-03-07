use std::str::FromStr;

use axum::{debug_handler, extract::State, response::IntoResponse, Json};
use serde::Deserialize;

use super::{return_base_res, return_bool_res};
use crate::datalayer::Device;
use crate::datalayer::DeviceType;
use crate::datalayer::User;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct InputAddDevice {
    name: String,
    #[serde(rename = "type")]
    device_type: String,
    notification: String,
}

#[derive(Deserialize)]
pub struct InputAddUser {
    name: String,
    device: InputAddDevice,
}

#[debug_handler]
pub async fn add_user(
    State(state): State<AppState>,
    Json(payload): Json<InputAddUser>,
) -> impl IntoResponse {
    let handleer = || async {
        let mut user = User::build(payload.name)?;

        let device_type: DeviceType = payload.device.device_type.parse()?;

        user.add_device(
            payload.device.name,
            device_type,
            payload.device.notification,
        )?;

        Ok(())
    };

    Json(return_bool_res(handleer().await))
}
