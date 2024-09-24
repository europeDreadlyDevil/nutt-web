use serde::{Deserialize, Serialize};
use tracing_log::log::{log, Level};
use nutt_web::http::response::responder::Responder;
use nutt_web::{box_route, get, routes, NuttServer};
use nutt_web::http::response::{Response, ResponseBuilder};
use nutt_web::http::status::StatusCode;

#[derive(Serialize, Deserialize, Debug)]
struct Json {
    name: String
}

#[tokio::main]
async fn main() {
    NuttServer::new()
        .bind(("127.0.0.1", 8080))
        .routes(routes!(
            get!("/", hello, Json),
            get!("/bye", bye)
        ))
        .run().await
}

async fn hello(data: Json) -> Response {
    log!(Level::Info, "{:#?}", data);
    ResponseBuilder::new(StatusCode::Accepted, "data accepted").build()
}

async fn bye() -> Response {
    "bye".into_response()
}