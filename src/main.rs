use std::fs;
use warp::Filter;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    log::info!("Hello, world!");

    let hello = warp::path!(String).map(|name| {
        let contents = fs::read_to_string(format!("{}.txt", name))
            .expect("Something went wrong reading the file");

        return contents;
    });

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
