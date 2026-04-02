use axum::{
    routing::{get, post, delete, patch}, 
    Json, Router, 
    extract::{State, Path}, 
    http::StatusCode, 
    response::IntoResponse
};
use sqlx::SqlitePool;
use crate::logic;
use serde::Deserialize;

// --- ROUTER ---
pub fn create_router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/api/stock", get(get_stock_handler).post(add_stock_handler))
        .route("/api/stock/:id", delete(delete_stock_handler))
        .route("/api/stock/:id/reduce", patch(reduce_stock_handler))
        .route("/api/recipes", get(get_recipes_handler).post(create_recipe_handler))
        .route("/api/recipes/:id", get(get_recipe_handler).put(update_recipe_handler))
        .route("/api/orders", post(create_order_handler).get(get_orders_history_handler))
        .with_state(pool)
}

// --- HANDLERS STOCK ---
#[derive(Deserialize)]
struct AddProductPayload {
    name: String,
    category: String,
    brand: String,
    amount: f64,
    unit: String,
    price_cents: i32,
}

async fn get_stock_handler(State(pool): State<SqlitePool>) -> impl IntoResponse {
    match logic::get_full_stock(&pool).await {
        Ok(stock) => (StatusCode::OK, Json(stock)).into_response(),
        Err(e) => {
            eprintln!("Erreur SQL Stock : {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Erreur lors de la récupération du stock").into_response()
        }
    }
}

async fn add_stock_handler(
    State(pool): State<SqlitePool>,
    Json(payload): Json<AddProductPayload>,
) -> impl IntoResponse {
    match logic::add_product_to_stock(
        &pool, &payload.name, &payload.category, &payload.brand, 
        payload.amount, &payload.unit, payload.price_cents
    ).await {
        Ok(_) => (StatusCode::CREATED, "Produit ajouté").into_response(),
        Err(e) => {
            eprintln!("Erreur ajout stock : {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Erreur ajout").into_response()
        }
    }
}

async fn delete_stock_handler(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match logic::delete_product_from_stock(&pool, id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(), // 204 No Content est standard pour un DELETE réussi
        Err(e) => {
            eprintln!("Erreur suppression : {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Erreur suppression").into_response()
        }
    }
}

#[derive(Deserialize)]
struct ReduceStockPayload {
    amount: f64,
}

async fn reduce_stock_handler(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(payload): Json<ReduceStockPayload>,
) -> impl IntoResponse {
    match logic::reduce_stock(&pool, id, payload.amount).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

// --- HANDLERS RECIPES ---

#[derive(Deserialize)]
struct CreateRecipePayload {
    name: String,
    ingredients: Vec<(i64, f64)>,
}

async fn get_recipes_handler(State(pool): State<SqlitePool>) -> impl IntoResponse {
    match logic::get_all_recipes(&pool).await {
        Ok(recipes) => (StatusCode::OK, Json(recipes)).into_response(),
        Err(e) => {
            eprintln!("Erreur SQL Recipes : {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Erreur lors de la récupération des recettes").into_response()
        }
    }
}

async fn create_recipe_handler(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateRecipePayload>,
) -> impl IntoResponse {
    // On appelle ta fonction de logic.rs
    match logic::create_recipe(&pool, &payload.name, payload.ingredients).await {
        Ok(_) => (StatusCode::CREATED, "Recette créée avec succès").into_response(),
        Err(e) => {
            eprintln!("Erreur SQL Create Recipe : {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Impossible de créer la recette").into_response()
        }
    }
}
async fn get_recipe_handler(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match logic::get_recipe_by_id(&pool, id).await {
        Ok(recipe) => (StatusCode::OK, Json(recipe)).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Recette introuvable").into_response(),
    }
}

async fn update_recipe_handler(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(payload): Json<CreateRecipePayload>, // On réutilise le même payload que pour create
) -> impl IntoResponse {
    match logic::update_recipe(&pool, id, &payload.name, payload.ingredients).await {
        Ok(updated) => (StatusCode::OK, Json(updated)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

// --- HANDLERS ORDERS ---

#[derive(Deserialize)]
struct CreateOrderPayload {
    recipe_ids: Vec<i64>,
}

async fn create_order_handler(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateOrderPayload>,
) -> impl IntoResponse {
    match logic::place_order_ticket(&pool, payload.recipe_ids).await {
        Ok(_) => (StatusCode::CREATED, "Commande créée avec succès").into_response(),
        
        Err(e) => {
            eprintln!("Erreur lors de la commande : {}", e);
            (StatusCode::BAD_REQUEST, e).into_response()
        }
    }
}

async fn get_orders_history_handler(State(pool): State<SqlitePool>) -> impl IntoResponse {
    match logic::get_orders_history(&pool).await {
        Ok(history) => (StatusCode::OK, Json(history)).into_response(),
        Err(e) => {
            eprintln!("Erreur SQL Orders History : {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Erreur lors de la récupération de l'historique des commandes").into_response()
        }
    }
}
