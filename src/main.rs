mod templates;

use crate::templates::MyTemplate;
use askama::Template;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{response::Html, routing::get, Router};
use std::fs::File;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing_subscriber::{filter, prelude::*};

#[tokio::main]
async fn main() {
    let file = match File::create("debug.log") {
        Ok(file) => file,
        Err(error) => panic!("Error creating debug file: {:?}", error),
    };
    let debug_log = tracing_subscriber::fmt::layer().with_writer(Arc::new(file));

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .pretty()
                .with_filter(filter::LevelFilter::INFO)
                .and_then(debug_log),
        )
        .init();

    let app = Router::new()
        .route("/", get(handler))
        .nest_service("/assets", ServeDir::new("assets"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> impl IntoResponse {
    tracing::debug!("Hello, Debug World!");
    tracing::warn!("Hello, Warning World!");
    let template = MyTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
