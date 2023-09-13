#![feature(impl_trait_in_assoc_type)]
use my_redis::{LogLayer, FilterLayer};
use std::{net::SocketAddr, collections::HashMap, sync::{Arc, Mutex}, string};
use my_redis::{S};
#[volo::main]
async fn main() {
    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let addr = volo::net::Address::from(addr);
    volo_gen::volo::redis::ItemServiceServer::new(S{ map: Mutex::new(HashMap::new())})
        .layer_front(LogLayer)
        .layer_front(FilterLayer)
        .run(addr)
        .await
        .unwrap();
}