use axum::Router;
use axum::extract::Path;
use axum::routing::get;
use axum::response::IntoResponse;
use std::net::SocketAddr;

async fn other_handler() -> impl axum::response::IntoResponse {
    "some other handler".into_response()
}

async fn root_handler(_: Option<Path<String>>) -> impl axum::response::IntoResponse {
    "Hello, World!".into_response()
}

fn build_nested_handler() -> Router {
    Router::new()
        .route("/:id", get(root_handler))
        .route("/other", get(other_handler))
        .fallback(get(root_handler))
}

#[tokio::main]
async fn main() {
    // With axum-0.6.11, `curl -v http://localhost:3000/nest/` returns 200 + "Hello World",
    // and with axum-0.6.20, the same curl command returns 404 + empty response.
    let nested_handlers = build_nested_handler();
    let app = Router::new().nest("/nest", nested_handlers);

    let port = 3000;

    let addr: SocketAddr = ([0, 0, 0, 0], port).into();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
