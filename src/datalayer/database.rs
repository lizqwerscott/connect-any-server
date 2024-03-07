use serde::{Deserialize, Serialize};

use super::DeviceType;

use crate::utils::database::{
    database_delete, database_insert, database_insert_no_id, database_select,
    database_select_single, get_database_connection,
};
use crate::utils::{ba_error, BDEResult};

#[derive(Serialize, Deserialize)]
pub struct DatabaseUser {
    pub id: u64,
    pub name: String,
}

impl DatabaseUser {
    pub fn insert_user(name: String) -> BDEResult<u64> {
        // Insert user into database
        let id = database_insert("users", vec!["name"], (name.clone(),))?;

        Ok(id)
    }

    pub fn delete_user(&self) -> BDEResult<()> {
        // Delete user from database
        database_delete("users", format!("id == {}", self.id))
    }

    pub fn find_user(name: String) -> BDEResult<Option<Self>> {
        // Find user in database
        let user = database_select::<Self>("users", Some(format!("name == '{}'", name)))?;

        Ok(user.into_iter().next())
    }

    pub fn get_user(id: u64) -> BDEResult<Option<Self>> {
        Ok(database_select_single("users", id)?)
    }
}

#[derive(Serialize, Deserialize)]
pub struct DatabaseDevice {
    pub id: u64,
    name: String,
    notification: String,
    #[serde(rename = "type")]
    device_type: DeviceType,
}

impl DatabaseDevice {
    pub fn insert_device(
        name: String,
        device_type: DeviceType,
        notification: String,
    ) -> BDEResult<Self> {
        // Insert device into database
        let id = database_insert(
            "devices",
            vec!["name", "notification", "type"],
            (name.clone(), notification.clone(), device_type),
        )?;

        Ok(DatabaseDevice {
            id,
            name,
            notification,
            device_type,
        })
    }

    pub fn delete_device(&self) -> BDEResult<()> {
        // Delete device from database
        database_delete("devices", format!("id == {}", self.id))
    }

    pub fn find_device(name: String, device_type: DeviceType) -> BDEResult<Option<Self>> {
        // Find device in database
        let device = database_select::<Self>(
            "devices",
            Some(format!("name == '{}' and type == '{}'", name, device_type)),
        )?;

        Ok(device.into_iter().next())
    }
}

#[derive(Serialize, Deserialize)]
pub struct DatabaseUserDevice {
    id: u64,
    user_id: u64,
    device_id: u64,
}

impl DatabaseUserDevice {
    pub fn insert_user_device(user_id: u64, device_id: u64) -> BDEResult<()> {
        // Insert user device into database
        database_insert_no_id(
            "user_device",
            vec!["user_id", "device_id"],
            (user_id, device_id),
        )?;

        Ok(())
    }

    pub fn delete_user_device(&self) -> BDEResult<()> {
        // Delete user device from database
        database_delete("user_device", format!("id == {}", self.id))
    }

    pub fn get_user_devices(user_id: u64) -> BDEResult<Vec<DatabaseDevice>> {
        let mut all_data: Vec<DatabaseDevice> = Vec::new();
        let conn = get_database_connection()?;

        let sql_command = format!("SELECT * FROM devices JOIN user_device ON devices.id = user_device.device_id where user_device.user_id = {}", user_id);

        let mut stmt = conn.prepare(sql_command.as_str())?;

        let data_iter = serde_rusqlite::from_rows::<DatabaseDevice>(stmt.query([])?);

        for data in data_iter {
            all_data.push(data?);
        }

        Ok(all_data)
    }

    pub fn get_device_users(device_id: u64) -> BDEResult<Vec<DatabaseUser>> {
        let mut all_data: Vec<DatabaseUser> = Vec::new();
        let conn = get_database_connection()?;

        let sql_command = format!("SELECT * FROM users JOIN user_device ON users.id = user_device.user_id where user_device.device_id = {}", device_id);

        let mut stmt = conn.prepare(sql_command.as_str())?;

        let data_iter = serde_rusqlite::from_rows::<DatabaseUser>(stmt.query([])?);

        for data in data_iter {
            all_data.push(data?);
        }

        Ok(all_data)
    }
}
