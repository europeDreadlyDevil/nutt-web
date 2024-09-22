use serde::Serialize;
use nutt_web::http::response::responder::Responder;
use nutt_web::{get, NuttServer};
use nutt_web::router::route::Route;

#[tokio::main]
async fn main() {
    let mut routes = Vec::new();
    routes.push(get!("/", hello_world));
    NuttServer::new()
        .routes(routes)
        .bind(("127.0.0.1", 8080))
        .run().await
}


async fn hello_world() -> impl Responder<String> {
    "Hello, World".to_string()
}