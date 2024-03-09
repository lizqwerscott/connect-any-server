use std::collections::HashMap;
use std::net::SocketAddr;
use std::ops::ControlFlow;

use serde::Deserialize;

use axum::{
    extract::connect_info::ConnectInfo, extract::ws, extract::State, response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};

use crate::datalayer::clipboard::Clipboard;
use crate::datalayer::{InputDevice, User};

use crate::state::{AppState, ClipboardData};

use crate::utils::{ArcBroadcastSender, ArcMutex, BDEResult};

#[derive(Deserialize)]
pub struct WsInitMessage {
    device: InputDevice,
    #[serde(rename = "type")]
    message_type: String,
}

pub async fn ws_handler(
    ws: ws::WebSocketUpgrade,
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    tracing::info!("ws: {addr} connected.");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| {
        handle_socket(
            socket,
            addr,
            state.clipboard_datas.clone(),
            state.client_n.clone(),
        )
    })
}

async fn get_ws_tx(
    user_id: u64,
    clipboard_data_arc: ArcMutex<HashMap<u64, ClipboardData>>,
) -> ArcBroadcastSender<Clipboard> {
    let mut clipboard_datas = clipboard_data_arc.lock().await;
    let clipboard_data = clipboard_datas
        .entry(user_id)
        .or_insert(ClipboardData::new());
    clipboard_data.ws_tx.clone()
}

fn process_init_message(msg: ws::Message) -> BDEResult<u64> {
    if let ws::Message::Text(text) = msg {
        tracing::info!("init message: {}", text);
        let data: WsInitMessage = serde_json::from_str(&text)?;

        if data.message_type == "init" {
            if let Ok(device) = data.device.parse() {
                if let Ok(user) = User::find_user_from_device(&device) {
                    return Ok(user.id);
                }
            }
        }
    }

    Err("init message error".into())
}

async fn handle_socket(
    mut socket: ws::WebSocket,
    who: SocketAddr,
    clipboard_data: ArcMutex<HashMap<u64, ClipboardData>>,
    client_n: ArcMutex<u8>,
) {
    // 建立链接, 将 ip 和 device id 对上号, 找到这个对应的 user, 如果发现有问题, 就断开链接, 返回错误信息
    // 让后等着接收消息, 如果接收到消息, 就将消息发送到对应的 user 的 ws 通道里面去

    // 接受初始化消息

    let user_id = if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match process_init_message(msg) {
                Ok(user_id) => user_id,
                Err(err) => {
                    tracing::error!("client {who} disconnectd: {err}");
                    return;
                }
            }
        } else {
            tracing::error!("client {who} disconnectd");
            return;
        }
    } else {
        tracing::error!("client {who} disconnectd");
        return;
    };

    let ws_tx = get_ws_tx(user_id, clipboard_data.clone()).await;

    let (mut sender, mut receiver) = socket.split();

    let mut ws_rx = ws_tx.subscribe();

    let mut send_ws_msg = tokio::spawn(async move {
        while let Ok(msg) = ws_rx.recv().await {
            let data = serde_json::to_string(&msg).unwrap();

            match sender.send(ws::Message::Text(data)).await {
                Ok(()) => {}
                Err(err) => {
                    tracing::error!(
                        "websocket send message error: {} with {}",
                        err.to_string(),
                        msg.data
                    );
                }
            }
        }
    });

    // This second task will receive messages from client and print them on server console
    let mut recv_task = tokio::spawn(async move {
        let mut cnt = 0;
        while let Some(Ok(msg)) = receiver.next().await {
            cnt += 1;
            // print message and break if instructed to do so
            if process_message(msg, who).await.is_break() {
                break;
            }
        }
        cnt
    });

    {
        let mut client_n = client_n.lock().await;
        *client_n += 1;
    }

    tokio::select! {
        _ = (&mut send_ws_msg) => {
            recv_task.abort();
        },
        rv_r = (&mut recv_task) => {
            match rv_r {
                Ok(r) => tracing::info!("Received {r} messages"),
                Err(r) => tracing::info!("Error receiving messages {r:?}")
            }
            send_ws_msg.abort();
        },
    }

    {
        let mut client_n = client_n.lock().await;
        *client_n -= 1;
    }

    // returning from the handler closes the websocket connection
    tracing::info!("Websocket context {who} destroyed");
}

async fn process_message(msg: ws::Message, who: SocketAddr) -> ControlFlow<(), ()> {
    match msg {
        ws::Message::Text(t) => {
            tracing::info!(">>> {who} sent str: {t:?}");
        }
        ws::Message::Binary(d) => {
            tracing::info!(">>> {} sent {} bytes: {:?}", who, d.len(), d);
        }
        ws::Message::Close(c) => {
            if let Some(cf) = c {
                tracing::info!(
                    ">>> {} sent close with code {} and reason `{}`",
                    who,
                    cf.code,
                    cf.reason
                );
            } else {
                tracing::info!(">>> {who} somehow sent close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }
        _ => {}
    }

    ControlFlow::Continue(())
}
