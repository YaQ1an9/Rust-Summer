use std::net::SocketAddr;
use axum::body;
use axum::extract::FromRequest;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Form, Router,
};
use faststr::FastStr;
use my_redis::DEFAULT_ADDR;
use serde::Deserialize;
use volo_gen::volo::redis::{ItemServiceClient, ItemServiceClientBuilder};

type RpcClient = ItemServiceClient;
type RpcClientBuilder = ItemServiceClientBuilder;
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let addr: SocketAddr = DEFAULT_ADDR.parse().unwrap();
    let rpc_cli = RpcClientBuilder::new("my_redis")
        .address(addr)
        .build();
    let app = Router::new()
        .route("/ping", get(ping).with_state(rpc_cli.clone()))
        .route("/get/:key", get(get_key).with_state(rpc_cli.clone()))
        .route(
            "/set",
            get(show_set_form).post(set_key).with_state(rpc_cli.clone()))
        .route("/del", 
        get(show_del_form).post(del_key).with_state(rpc_cli.clone()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    tracing::debug!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
async fn ping(State(rpc_cli): State<RpcClient>) -> impl IntoResponse {
    let req = volo_gen::volo::redis::GetItemRequest {
        cmd: volo_gen::volo::redis::RedisCommand::Ping,
        args: None,
    };
    // println!("req: {:?}", req);
    let resp = rpc_cli.get_item(req).await.unwrap();
    match resp {
        volo_gen::volo::redis::GetItemResponse { ok, data } => {
            if ok {
                println!("Pong");
                Html(format!("Pong"))
            } else {
                Html(format!("Error: {:?}", data))
            }
        }
    }
}
async fn get_key(
        Path(key): Path<String>,
        State(rpc_cli): State<RpcClient>,
    ) -> impl IntoResponse {
    let req = volo_gen::volo::redis::GetItemRequest {
        cmd: volo_gen::volo::redis::RedisCommand::Get,
        args: Some(vec![key.into()]),
    };
    println!("req: {:?}", req);
    let resp = rpc_cli.get_item(req).await.unwrap();
    match resp {
        volo_gen::volo::redis::GetItemResponse { ok, data } => {
            if ok {
                println!("{:?}", data.unwrap());
                Html(format!("Found"))
            } else {
                println!("Error: {:?}", data);
                Html(format!("Not Found"))
            }
        }
    }
}

async fn show_set_form() -> Html<&'static str> {
    println!("this is show_set_form");
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/set" method="post">
                    <label for="key">
                        Enter key:
                        <input type="text" name="key">
                    </label>
                    <input type="submit" value="Subscribe!">
                </form>
            </body>
        </html>
        "#,
    )
}

async fn set_key(
        State(rpc_cli): State<RpcClient>,
        body: String
    ) -> impl IntoResponse {
    let args = body.split_whitespace().collect::<Vec<&str>>();
    let req = volo_gen::volo::redis::GetItemRequest {
        cmd: volo_gen::volo::redis::RedisCommand::Set,
        args: Some(args.iter().map(|s| FastStr::from(s.to_string())).collect()),
    };
    let resp = rpc_cli.get_item(req).await.unwrap();
    match resp {
        volo_gen::volo::redis::GetItemResponse { ok, data } => {
            if ok {
                println!("Set");
                Html(format!("Set key"))
            } else {
                println!("Error: {:?}", data);
                Html(format!("Error: {:?}", data))
            }
        }
    }
}

async fn show_del_form() -> Html<&'static str> {
    println!("this is show_del_form");
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/del" method="post">
                    <label for="key">
                        Enter key:
                        <input type="text" name="key">
                    </label>
                    <input type="submit" value="Subscribe!">
                </form>
            </body>
        </html>
        "#,
    )
}

async fn del_key(
        State(rpc_cli): State<RpcClient>,
        body: String
    ) -> impl IntoResponse {
    let args = body.split_whitespace().collect::<Vec<&str>>();
    let req = volo_gen::volo::redis::GetItemRequest {
        cmd: volo_gen::volo::redis::RedisCommand::Del,
        args: Some(args.iter().map(|s| FastStr::from(s.to_string())).collect()),
    };
    let resp = rpc_cli.get_item(req).await.unwrap();
    match resp {
        volo_gen::volo::redis::GetItemResponse { ok, data } => {
            if ok {
                println!("Del");
                Html(format!("Del key"))
            } else {
                println!("Error: {:?}", data);
                Html(format!("Error: {:?}", data))
            }
        }
    }
}
