use axum::{debug_handler, extract::Query, response::IntoResponse, Json};
use serde::Deserialize;

use super::{return_base_res, return_bool_res};
use crate::datalayer::DeviceType;
use crate::datalayer::User;
use crate::utils::ba_error;

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
pub async fn add_user(Json(payload): Json<InputAddUser>) -> impl IntoResponse {
    let handler = || {
        let mut user = User::build(payload.name)?;

        let device_type: DeviceType = payload.device.device_type.parse()?;

        user.add_device(
            payload.device.name,
            device_type,
            payload.device.notification,
        )?;

        Ok(())
    };

    Json(return_bool_res(handler()))
}

#[derive(Deserialize)]
pub struct InputGetUser {
    name: String,
}

#[debug_handler]
pub async fn get_user_device(username: Query<InputGetUser>) -> impl IntoResponse {
    let handler = || {
        if let Some(user) = User::find_user(username.name.clone())? {
            Ok(user)
        } else {
            Err(ba_error("not find user"))
        }
    };

    Json(return_base_res(handler()))
}
