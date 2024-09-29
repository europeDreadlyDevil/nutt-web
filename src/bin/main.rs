use nutt_web::http::response::responder::Responder;
use nutt_web::http::response::Response;
use nutt_web::router::route::Route;
use nutt_web::router::{delete, get, post, put};
use nutt_web::state::State;
use nutt_web::{routes, NuttServer};
use serde::{Deserialize, Serialize};
use tracing_log::log::{log, Level};

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
    server: NuttServer,
}

impl App {
    pub fn new() -> Self {
        let data = State::new(10);
        Self {
            server: NuttServer::new()
                .bind(("127.0.0.1", 8080))
                .routes(routes!(
                    App::hello,
                    App::post_data,
                    App::put_data,
                    App::delete_data
                ))
                .state(("num".into(), data)),
        }
    }
    pub async fn run(self) {
        self.server.run().await
    }

    #[get("/")]
    async fn hello() -> Response {
        "hello".into_response()
    }

    #[post("/data")]
    async fn post_data(data: Data) -> Response {
        log!(Level::Info, "{data:?}");
        "data accepted".into_response()
    }

    #[put("/update")]
    async fn put_data(data: Data) -> Response {
        log!(Level::Info, "Update: {data:?}");
        "data updated".into_response()
    }

    #[delete("/delete")]
    async fn delete_data(data: Data) -> Response {
        log!(Level::Info, "Delete: {data:?}");
        "data deleted".into_response()
    }
}
