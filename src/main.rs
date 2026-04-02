mod db;
mod logic;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    let pool = db::init_db().await;

    let app = routes::create_router(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("🚀 Bar ouvert sur http://localhost:8080");
    
    axum::serve(listener, app).await.unwrap();
}
