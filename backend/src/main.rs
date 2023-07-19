use axum::routing::get;
use std::net::SocketAddr;

async fn root() -> &'static str {
    "Hello, world!"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let app = axum::Router::new().route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::debug!("Listening on {}", addr);

    axum::Server::bind(&addr).serve(app.into_make_service())
        .await?;

    Ok(())
}
