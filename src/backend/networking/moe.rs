use futures_util::{FutureExt, future::BoxFuture};
use rust_socketio::{
    Payload,
    asynchronous::{Client, ClientBuilder},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{str::FromStr, time::Duration};

#[derive(Debug, Serialize)]
struct Auth {
    username: String,
    password: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
struct AuthForm {
    body: Auth,
    authorization: String,
    onlineAuthorization: String,
}

pub async fn main(config: crate::KaraConfig) {
    let callback = |payload: Payload, socket: Client| {
        async move {
            match payload {
                Payload::Text(values) => println!("Received: {:#?}", values),
                Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
                Payload::String(str) => println!("Received: {}", str),
            }
            socket
                .emit("test", json!({"got ack": true}))
                .await
                .expect("Server unreachable!")
        }
        .boxed()
    };

    let ack_callback = |message: Payload, _: Client| {
        async move {
            println!("Yehaa! My ack got acked?");
            println!("Ack data: {:#?}", message);
        }
        .boxed()
    };

    auth(config, callback, ack_callback).await;
}

async fn auth<F, G>(config: crate::KaraConfig, callback: F, ack_callback: G)
where
    F: for<'a> FnMut(Payload, Client) -> BoxFuture<'static, ()> + Send + Sync + 'static,
    G: FnMut(Payload, rust_socketio::asynchronous::Client) -> BoxFuture<'static, ()>
        + Send
        + Sync
        + 'static,
{
    let url = config.moe_server;
    let auth = Auth {
        username: String::from_str("hello").unwrap(),
        password: String::from_str("superpassword").unwrap(),
    };

    // let auth_form = AuthForm {
    //     body: auth,
    //     authorization: String::new(),
    //     onlineAuthorization: String::new(),
    // };

    println!("Authing at: {}", url);
    let socket = ClientBuilder::new(url)
        .namespace("/")
        .on("error", |err, _| {
            async move { eprintln!("Error: {:#?}", err) }.boxed()
        })
        .connect()
        .await
        .expect("Connection Failed");

    let payload = json!({"body": {
        "username": "martinmimi",
    }});

    socket
        .emit_with_ack("loginGuest", payload, Duration::from_secs(2), ack_callback)
        .await
        .expect("Server unreachable");

    tokio::time::sleep(Duration::from_secs(10)).await;

    socket.disconnect().await.expect("Disconnect failed");
    println!("Server Disconnected")
}
