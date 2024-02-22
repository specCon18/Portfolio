use tower_http::services::ServeDir;
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod router;
mod handlers;
mod templates;

#[tokio::main]
async fn main() {
    setup_tracing();

    let assets_path = env::current_dir().unwrap().join("assets");
    let app = router::router(&assets_path);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    println!("Listening on port 3000");
}

fn setup_tracing(){  
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "portfolio=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
    

