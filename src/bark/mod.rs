use reqwest::{Client, Url};

use crate::{
    datalayer::DeviceType,
    utils::{ba_error, BDEResult},
};

pub async fn send_bark(
    bark_id: String,
    device_name: String,
    device_type: DeviceType,
    data: String,
) -> BDEResult<()> {
    let base_url = format!(
        "https://api.day.app/{}/Clipboard from:{}({})/",
        bark_id, device_name, device_type
    );

    let client = Client::new();

    let url = Url::parse_with_params(
        base_url.as_str(),
        &[
            ("autoCopy", "1"),
            ("automaticallyCopy", "1"),
            ("copy", data.as_str()),
        ],
    )?;

    let res = client.get(url).send().await?;

    if res.status().is_success() {
        Ok(())
    } else {
        let res_data = res.text().await?;

        Err(ba_error(
            format!("Failed to send bark: {}", res_data).as_str(),
        ))
    }
}
