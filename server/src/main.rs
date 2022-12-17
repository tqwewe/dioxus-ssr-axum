use app::app;
use axum::{
    http::StatusCode,
    response::Html,
    routing::{get, get_service},
    Router,
};
use dioxus::prelude::*;
use tokio::fs;
use tower_http::services::ServeDir;

const DIST: &str = "app/dist";

#[tokio::main]
async fn main() {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);

    let serve_dir = get_service(ServeDir::new(DIST)).handle_error(|_| async move {
        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
    });
    let app = Router::new()
        .route("/", get(app_endpoint))
        .nest_service("/assets", serve_dir);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// fn serve()

async fn app_endpoint() -> Html<String> {
    let index = fs::read_to_string(format!("{DIST}/index.html"))
        .await
        .unwrap();
    let (prefix, suffix) = index.split_once(r#"<div id="main">"#).unwrap();

    let mut app = VirtualDom::new(app::app);
    let _ = app.rebuild();

    let html = dioxus::ssr::render_vdom(&app);
    Html(format!(r#"{prefix}<div id="main">{html}{suffix}"#))
}
