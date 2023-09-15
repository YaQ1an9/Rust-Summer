#![feature(impl_trait_in_assoc_type)]
use std::hash::Hash;
use std::thread;
use std::{net::SocketAddr, collections::HashMap, string, path::Path};
use std::sync::{Arc, Mutex};
use std::io::{self, BufRead, Read};
use my_redis::{S, LogLayer, FilterLayer, Proxy, Range};
use std::fs::File;
use my_redis::Type;
use toml::Value;
use my_redis::Type::{Slave, Master};
// use async_task::spawn;
const Log_path: &str = "./DataBase/test.log";

#[volo::main]
async fn main() {
    tracing_subscriber::fmt::init();
    //通过循环启动多个redis实例
    let config_path = "./Config/redis_1.conf";
    // let config_path = "/Users/dn_csj/RUST/Rust-Summer/Homework/hw6/my_redis/Config/redis_1.conf";
    let Proxy = read_conf(config_path);

    let Master_addr = Proxy.addr_master.clone(); 
    let HashMap = read_log(Log_path);

    let addr: SocketAddr = Proxy.proxy_addr.parse().unwrap();
    let mut all_port = Vec::new();
    let mut slave = Vec::new();
    let mut num = Proxy.severs_addr.lock().unwrap().len();
    for value in Proxy.severs_addr.lock().unwrap().values() {
        all_port.push(value.clone());
        if value != &Master_addr {
            slave.push(value.clone());
        }
    }
    println!("tmp: {:?}", all_port);
    for i in 0..num {
        let _log_path = Log_path.to_string();
        let addr = all_port[i].clone();
        let mut _type = Master;
        if addr != Proxy.addr_master { _type = Slave; }
        let S = S {
            _type,
            _slave: Mutex::new(Some(slave.clone())),
            map: Mutex::new(HashMap.clone()),
            _log_path: _log_path.to_string(),
        };
        let addr: SocketAddr = addr.parse().unwrap();
        let addr = volo::net::Address::from(addr);
        tokio::spawn({ volo_gen::volo::redis::ItemServiceServer::new(S)
        // .layer_front(LogLayer)
        .layer_front(FilterLayer)
        .run(addr)
        });
    }

    let addr = Proxy.proxy_addr.clone();
    println!("proxy_addr: {}", addr);
    let addr: SocketAddr = addr.parse().unwrap();
    let addr = volo::net::Address::from(addr);
    tokio::spawn( { volo_gen::volo::redis::ItemServiceServer::new(Proxy)
    // .layer_front(LogLayer)
    .layer_front(FilterLayer)
    .run(addr)
    });

    tokio::signal::ctrl_c().await.unwrap();

}
fn read_log(file_path: &str) -> HashMap<String, String> {
    let mut HashMap = HashMap::new();
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(line) = line {
                let mut iter = line.split_whitespace();
                let key = iter.next().unwrap().to_string();
                let value = iter.next().unwrap().to_string();
                HashMap.insert(key, value);
            }
        }
    }
    HashMap
}   
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn read_conf(filepath: &str) -> Proxy {
    // 读取配置文件
    let mut file = File::open(filepath).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // 解析配置文件
    let config: Value = toml::from_str(&contents).unwrap();

    // 获取配置信息并构建 Proxy 结构体
    let proxy_addr = config["proxy"]["proxy_addr"].as_str().unwrap().to_owned();
    let addr_master = config["proxy"]["addr_master"].as_str().unwrap().to_owned();

    let server_num = config["proxy"]["server_num"].as_integer().unwrap() as usize;
    let mut servers_addr = HashMap::new();

    for n in 1..=server_num {
        let server = &config[&format!("server{}", n)];

        let start = server["start"].as_integer().unwrap() as u32;
        let end = server["end"].as_integer().unwrap() as u32;
        let addr = server["addr"].as_str().unwrap().to_owned();

        servers_addr.insert(Range { start, end }, addr);
    }

    let proxy = Proxy {
        proxy_addr,
        severs_addr: Mutex::new(servers_addr),
        addr_master,
    };
    proxy
}