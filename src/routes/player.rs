use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    Extension,
};
use axum::{headers, TypedHeader};
use redis::aio::ConnectionManager;
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};
use uuid::Uuid;

use std::borrow::Cow;
use std::net::SocketAddr;
use std::ops::ControlFlow;

// allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::CloseFrame;

// allows to split the websocket stream into separate TX and RX branches
use futures::{sink::SinkExt, stream::StreamExt};

use crate::{config::AppContext, core::user::User};

#[derive(Deserialize, Debug)]
struct PlayerState {
    episode_id: String,
    player_time: u64,
}

/// The handler for the HTTP request (this gets called when the HTTP GET lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
pub async fn player_ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(mut state): State<AppContext>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    info!("`{user_agent}` at {addr} connected.");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    let mut redis_connection = state.redis_manager;
    ws.on_upgrade(move |socket| async move {
        handle_socket(socket, addr, &mut redis_connection, user.id).await
    })
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(
    socket: WebSocket,
    who: SocketAddr,
    con: &mut ConnectionManager,
    user_id: Uuid,
) {
    let (mut tx, mut rx) = socket.split();
    while let Some(Ok(msg)) = rx.next().await {
        if let Message::Text(text) = msg {
            // Decode our message and warn if it's something we don't know about
            if let Ok(player_state) = serde_json::from_str::<PlayerState>(&text) {
                // Apply the state
                info!("Storing {:#?} for User {user_id}", player_state);
                let result = redis::cmd("SET")
                    .arg(format!("User:{user_id}:PlayerState"))
                    .arg(text)
                    .query_async::<_, ()>(con)
                    .await;
                info!("Result: {:#?}", result);
            } else if text == "get_state" {
                info!("Getting state for User {user_id}");
                let result = redis::cmd("GET")
                    .arg(format!("User:{user_id}:PlayerState"))
                    .query_async::<_, String>(con)
                    .await;
                if let Ok(result) = result {
                    let send_result = tx.send(Message::Text(result.clone())).await;
                    info!("Redis result: {:#?}", result);
                }
            } else {
                warn!("Unknown action received: {}", text);
            }
        }
    }
    // returning from the handler closes the websocket connection
    info!("Websocket context {who} destroyed");
}
