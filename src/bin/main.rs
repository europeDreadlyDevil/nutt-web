use std::collections::HashMap;
use std::hash::{DefaultHasher, Hasher};
use nutt_web::http::response::responder::Responder;
use nutt_web::http::response::Response;
use nutt_web::modules::router::route::Route;
use nutt_web::modules::router::{delete, get, post, put};
use nutt_web::modules::state::State;
use nutt_web::{routes, state, NuttServer};
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

#[derive(Debug, Serialize, Deserialize, Clone)]
struct NewStateJson {
    num: i32
}

struct App {
    server: NuttServer,
}

impl App {
    pub fn new() -> Self {
        let num = State::new(10);
        let tokens: State<HashMap<String, String>> = State::new(HashMap::new());
        Self {
            server: NuttServer::new()
                .bind(("127.0.0.1", 8080))
                .routes(routes!(
                    App::hello,
                    App::post_data,
                    App::put_data,
                    App::delete_data,
                    App::get_state,
                    App::change_state,
                    App::login_user,
                ))
                .state(state!(num))
                .state(state!(tokens)),
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

    #[post("/change-state")]
    async fn change_state(num: State<i32>, state: NewStateJson) -> Response {
        *num.write() = state.num;
        "success".into_response()
    }

    #[get("/get-state")]
    async fn get_state(num: State<i32>) -> Response {
        num.read().into_response()
    }

    #[post("/login")]
    async fn login_user(data: Data, tokens: State<HashMap<String, String>>) -> Response {
        let mut h = DefaultHasher::default();
        h.write(data.password.as_bytes());
        tokens.write().insert(data.login.clone(), h.finish().to_string());
        log!(Level::Info, "{tokens:?}");
        "success".into_response()
    }
}
