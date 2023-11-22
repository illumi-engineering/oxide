use warp::Filter;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    port: Option<i32>,
    open: Option<bool>,
}

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let routes = hello;

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
