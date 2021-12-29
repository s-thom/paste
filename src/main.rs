use warp::Filter;
extern crate dotenv;
extern crate env_logger;
extern crate log;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    log::info!("Hello, world!");
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
