use reqwest::{header::CONTENT_TYPE, StatusCode};

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let pong = client
        .get("http://localhost:3030/ping")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap(); 

    let body = "hello world";
    let set = client
        .post("http://localhost:3030/set")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    assert_eq!(set.status(), 200);

    let get = client
        .get("http://localhost:3030/get/hello")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(get, "Found");

    let body = "hello";
    let del = client
        .post("http://localhost:3030/del")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    assert_eq!(del.status(), 200);
    let get = client
        .get("http://localhost:3030/get/hello")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(get, "Not Found");
    println!("test success");
    
}