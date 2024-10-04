use nutt_web::http::response::responder::Responder;
use nutt_web::http::response::{Response, ResponseBuilder};
use nutt_web::http::status::StatusCode;
use nutt_web::modules::router::route::Route;
use nutt_web::modules::router::{delete, get, post, put};
use nutt_web::modules::session::cookie_session::{CookieSession, SessionId};
use nutt_web::modules::session::SessionType;
use nutt_web::modules::state::State;
use nutt_web::{routes, state, NuttServer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hasher};
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

#[derive(Serialize, Deserialize, Clone, Debug)]
struct UserId {
    id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct NewStateJson {
    num: i32,
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
                    App::auth_user
                ))
                .session(SessionType::Cookie)
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
    async fn login_user(data: Data, mut session: CookieSession) -> Response {
        let id = session.create_new_session();
        session.set_data_by_id(id.clone(), ("login", data.login));
        session.set_data_by_id(id.clone(), ("password", data.password));
        id.to_string().into_response()
    }

    #[get("/auth")]
    async fn auth_user(id: UserId, mut session: CookieSession) -> Response {
        let data = session.get_session_data(SessionId::from(id.id));
        if let Some(data) = data {
            ResponseBuilder::new(
                StatusCode::Accepted,
                Data {
                    login: data.get::<String>("login").unwrap().clone(),
                    password: data.get::<String>("password").unwrap().clone(),
                },
            )
            .build()
        } else {
            ResponseBuilder::new(StatusCode::UnAuthorized, "").build()
        }
    }
}
