
use serde::{Deserialize, Serialize};
use nutt_web::http::response::{Response, ResponseBuilder};
use nutt_web::{routes, NuttServer};
use nutt_web::router::{get};
use tracing_log::log::log;
use tracing_log::log::Level;
use nutt_web::http::response::responder::Responder;
use nutt_web::http::status::StatusCode;
use nutt_web::router::route::Route;
use nutt_web::state::State;

#[tokio::main]
async fn main() {
    let app = App::new();
    app.run().await;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Data {
    login: String,
    password: String,
}

struct App {
    server: NuttServer
}

impl App {
    pub fn new() -> Self {
        let data = State::new(10);
        Self {
            server: NuttServer::new()
                .bind(("127.0.0.1", 8080))
                .routes(routes!(App::hello))
                .state(("num".into(), data))
        }
    }
    pub async fn run(self) {
        self.server.run().await
    }

    #[get("/")]
    async fn hello(num: State<i32>, data: Data) -> Response {
        println!("{num:?} {:?}", data);
        ResponseBuilder::new(StatusCode::Accepted, data).build().into_response()
    }

}