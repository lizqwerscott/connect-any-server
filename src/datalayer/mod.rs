use rustsqlite_derive::ToSqlMacro;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use strum_macros::EnumString;

use crate::utils::BDEResult;

pub mod clipboard;

mod database;

#[derive(
    Deserialize, Serialize, EnumString, Display, ToSqlMacro, Clone, Copy, Debug, PartialEq, Eq,
)]
pub enum DeviceType {
    IOS,
    Android,
    Windows,
    Mac,
    Linux,
}

pub type Device = database::DatabaseDevice;

#[derive(Deserialize, Serialize)]
pub struct User {
    id: u64,
    name: String,
    devices: Vec<Device>,
}

impl User {
    pub fn build(name: String) -> BDEResult<Self> {
        // 尝试从数据库中查找用户
        if let Some(user) = database::DatabaseUser::find_user(name.clone())? {
            // 如果找到用户，则获取其设备信息
            let devices = database::DatabaseUserDevice::get_user_devices(user.id)?;

            // 返回用户及其设备信息
            return Ok(Self {
                id: user.id,
                name: user.name,
                devices,
            });
        }

        // 如果用户不存在，则将其插入数据库
        let id = database::DatabaseUser::insert_user(name.clone())?;

        Ok(User {
            id,
            name,
            devices: vec![],
        })
    }

    pub fn add_device(
        &mut self,
        name: String,
        device_type: DeviceType,
        notification: String,
    ) -> BDEResult<()> {
        if let None = database::DatabaseDevice::find_device(name.clone(), device_type)? {
            let device = database::DatabaseDevice::insert_device(name, device_type, notification)?;

            database::DatabaseUserDevice::insert_user_device(self.id, device.id)?;

            self.devices.push(device);
        }

        Ok(())
    }
}
