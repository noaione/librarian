use std::sync::Arc;

use axum::{
    extract::State,
    http::Uri,
    response::{Html, IntoResponse, Redirect},
    routing::get,
    Router,
};
use komga::KomgaClient;
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use urlencoding::encode;

include!(concat!(env!("OUT_DIR"), "/index_html.rs"));

mod komga;
mod routes;

#[derive(Clone)]
pub struct AppState {
    pub redis: Arc<redis::Client>,
    pub komga: Arc<KomgaClient>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "k-librarian=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv::dotenv().ok();

    std::env::var("TOKEN").expect("TOKEN not set");
    let komga_host = std::env::var("KOMGA_HOST").expect("KOMGA_HOST not set");
    let komga_username = std::env::var("KOMGA_USERNAME").expect("KOMGA_USERNAME not set");
    let komga_password = std::env::var("KOMGA_PASSWORD").expect("KOMGA_PASSWORD not set");

    let komga_client = KomgaClient::new(
        komga_host.clone(),
        komga_username.clone(),
        komga_password.clone(),
    );

    let redis_host = std::env::var("REDIS_HOST").expect("REDIS_HOST not set");
    let redis_port = std::env::var("REDIS_PORT").expect("REDIS_PORT not set");
    let redis_pass = std::env::var("REDIS_PASS").unwrap_or("".to_string());

    let mut redis_url = format!("redis://{}:{}", redis_host, redis_port);
    if redis_pass != "" {
        redis_url = format!("redis://:{}@{}:{}", redis_pass, redis_host, redis_port);
    }

    tracing::info!("🔌 Connecting to Redis at: {}", redis_url);
    let redis_client = redis::Client::open(redis_url).unwrap();

    tracing::info!("🔌 Connecting to Komga at: {}", komga_host);
    let komga_user = komga_client.get_me().await.unwrap();

    // Check if ADMIN role
    if !komga_user.roles.contains(&"ADMIN".to_string()) {
        panic!("Provided Komga user is not an ADMIN!")
    }

    let state = AppState {
        redis: Arc::new(redis_client),
        komga: Arc::new(komga_client),
    };

    let assets_dir = ServeDir::new("assets/assets");

    let app: Router = Router::new()
        .route("/", get(index))
        .route(
            "/favicon.ico",
            get(|_: State<AppState>| async { include_bytes!("../assets/favicon.ico").to_vec() }),
        )
        .route("/_/health", get(|| async { "ok" }))
        .nest("/api", routes::api(state.clone()))
        .nest_service("/assets", assets_dir)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::new().allow_origin(Any))
        .with_state(state);

    let app = app.fallback(handle_404);

    let host_at = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port_at = std::env::var("PORT").unwrap_or("5148".to_string());

    // run it
    let listener = TcpListener::bind(format!("{}:{}", host_at, port_at))
        .await
        .unwrap();
    tracing::info!(
        "🚀 Fast serving at: http://{}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap()
}

async fn handle_404(url: Uri) -> Redirect {
    let path = url.path().to_string();
    tracing::info!("404: {}", path);

    let redirect_url = format!("/?redirect={}", encode(&path));
    Redirect::to(&redirect_url)
}

async fn index(_: State<AppState>) -> impl IntoResponse {
    Html(INDEX_HTML)
}
