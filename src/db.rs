use sqlx::SqlitePool;

pub async fn init_db() -> SqlitePool {
    let database_url = "sqlite:bar.db";

    let pool = SqlitePool::connect(database_url)
        .await
        .expect("Erreur de connexion");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Le dossier migrations est introuvable ou le SQL est faux");

    pool
}