
use serde::Deserialize;
use nutt_web::http::response::Response;
use nutt_web::{routes, NuttServer};
use nutt_web::router::{get, post};
use tracing_log::log::log;
use tracing_log::log::Level;
use nutt_web::http::response::responder::Responder;
use nutt_web::router::route::Route;

#[tokio::main]
async fn main() {
    let app = App::new();
    app.run().await;
}

#[derive(Deserialize, Clone, Debug)]
struct Data {
    login: String,
    password: String,
}

struct App {
    server: NuttServer
}

impl App {
    pub fn new() -> Self {
        Self {
            server: NuttServer::new()
                .bind(("127.0.0.1", 8080))
                .routes(routes!(App::hello, App::post_data))
        }
    }
    pub async fn run(self) {
        self.server.run().await
    }

    #[get("/")]
    async fn hello() -> Response {
        "Hello, world!".into_response()
    }

    #[post("/data")]
    async fn post_data(data: Data) -> Response {
        log!(Level::Info, "Request data: {:?}", data);
        "Data accepted".into_response()
    }
}