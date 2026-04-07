mod db;
mod logic;
mod models;
mod routes;

use tokio::sync::broadcast;
use crate::routes::AppState;

#[tokio::main]
async fn main() {
    let pool = db::init_db().await;

    let (tx, _rx) = broadcast::channel::<String>(100);
    let state = AppState { pool, tx };

    let app = routes::create_router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("🚀 Bar ouvert sur http://localhost:8080");
    
    axum::serve(listener, app).await.unwrap();
}
