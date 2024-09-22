use nutt_web::http::response::responder::Responder;
use nutt_web::{box_route, routes, NuttServer};
use nutt_web::http::method::Method;
use nutt_web::http::response::Response;
use nutt_web::router::route::Route;

#[tokio::main]
async fn main() {
    NuttServer::new()
        .bind(("127.0.0.1", 8080))
        .routes(routes![
            Route::new(Method::GET, "/", box_route!(hello_world)),
            Route::new(Method::GET, "/bye", box_route!(bye_world))
        ])
        .run().await;

}


async fn hello_world() -> Response {
    "Hello, World".to_string().into_response()
}

async fn bye_world() -> Response {
    10.into_response()
}
