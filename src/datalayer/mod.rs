use rustsqlite_derive::ToSqlMacro;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use strum_macros::EnumString;

use crate::utils::ba_error;
use crate::utils::BDEResult;

pub mod clipboard;

mod database;

#[derive(
    Deserialize, Serialize, EnumString, Display, ToSqlMacro, Clone, Copy, Debug, PartialEq, Eq,
)]
pub enum DeviceType {
    Ios,
    Android,
    Windows,
    Mac,
    Linux,
}

pub type Device = database::DatabaseDevice;

#[derive(Deserialize)]
pub struct InputDevice {
    name: String,
    #[serde(rename = "type")]
    device_type: String,
}

impl InputDevice {
    pub fn parse(self) -> BDEResult<Device> {
        let device_type = self.device_type.parse()?;

        Device::get_device(self.name, device_type)
    }
}

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub devices: Vec<Device>,
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

    pub fn find_user(name: String) -> BDEResult<Option<Self>> {
        if let Some(user) = database::DatabaseUser::find_user(name.clone())? {
            // 如果找到用户，则获取其设备信息
            let devices = database::DatabaseUserDevice::get_user_devices(user.id)?;

            // 返回用户及其设备信息
            Ok(Some(Self {
                id: user.id,
                name: user.name,
                devices,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn find_user_from_device(device: &Device) -> BDEResult<Self> {
        if let Some(user) = database::DatabaseUserDevice::get_device_users(device.id)? {
            // 如果找到用户，则获取其设备信息
            let devices = database::DatabaseUserDevice::get_user_devices(user.id)?;

            // 返回用户及其设备信息
            Ok(Self {
                id: user.id,
                name: user.name,
                devices,
            })
        } else {
            Err(ba_error("device user not found"))
        }
    }

    pub fn add_device(
        &mut self,
        name: String,
        device_type: DeviceType,
        notification: String,
    ) -> BDEResult<()> {
        if database::DatabaseDevice::find_device(name.clone(), device_type)?.is_none() {
            let device = database::DatabaseDevice::insert_device(name, device_type, notification)?;

            database::DatabaseUserDevice::insert_user_device(self.id, device.id)?;

            self.devices.push(device);
        }

        Ok(())
    }
}
