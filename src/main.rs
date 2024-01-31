use axum::{response::Html, routing::get, Router};
use std::fs::File;
use std::sync::Arc;
use tracing_subscriber::{filter, prelude::*};

#[tokio::main]
async fn main() {
    // A layer that logs events to a file.
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
                .and_then(debug_log)
        )
        .init();

    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    tracing::debug!("Hello, Debug World!");
    tracing::warn!("Hello, Warning World!");
    Html("<h1>Hello, World!</h1>")
}
