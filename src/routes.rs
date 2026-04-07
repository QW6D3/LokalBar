use axum::{
    routing::{get, post, delete, patch}, 
    Json, Router, 
    extract::{State, Path}, 
    http::StatusCode, 
    response::IntoResponse,
    extract::ws::{WebSocketUpgrade, WebSocket},
};
use sqlx::SqlitePool;
use crate::logic;
use serde::Deserialize;
use tokio::sync::broadcast;
use futures_util::{sink::SinkExt, stream::StreamExt};
use tower_http::cors::{CorsLayer, Any};
// --- ROUTER ---
pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        // En développement, on peut être très permissif :
        .allow_origin(Any) 
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST, axum::http::Method::OPTIONS])
        .allow_headers(Any);

    Router::new()
        .route("/api/stock", get(get_stock_handler).post(add_stock_handler))
        .route("/api/stock/:id", delete(delete_stock_handler))
        .route("/api/stock/:id/reduce", patch(reduce_stock_handler))
        .route("/api/recipes", get(get_recipes_handler).post(create_recipe_handler))
        .route("/api/recipes/:id", get(get_recipe_handler).put(update_recipe_handler))
        .route("/api/orders", post(create_order_handler).get(get_orders_history_handler))
        .route("/ws", get(ws_handler))
        .layer(cors)
        .with_state(state)
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

async fn get_stock_handler(State(state): State<AppState>) -> impl IntoResponse {
    match logic::get_full_stock(&state.pool).await {
        Ok(stock) => (StatusCode::OK, Json(stock)).into_response(),
        Err(e) => {
            eprintln!("Erreur SQL Stock : {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Erreur lors de la récupération du stock").into_response()
        }
    }
}

async fn add_stock_handler(
    State(state): State<AppState>,
    Json(payload): Json<AddProductPayload>,
) -> impl IntoResponse {
    match logic::add_product_to_stock(
        &state.pool, &payload.name, &payload.category, &payload.brand, 
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
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match logic::delete_product_from_stock(&state.pool, id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
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
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<ReduceStockPayload>,
) -> impl IntoResponse {
    match logic::reduce_stock(&state.pool, id, payload.amount).await {
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

async fn get_recipes_handler(State(state): State<AppState>) -> impl IntoResponse {
    match logic::get_all_recipes(&state.pool).await {
        Ok(recipes) => (StatusCode::OK, Json(recipes)).into_response(),
        Err(e) => {
            eprintln!("Erreur SQL Recipes : {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Erreur lors de la récupération des recettes").into_response()
        }
    }
}

async fn create_recipe_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateRecipePayload>,
) -> impl IntoResponse {
    match logic::create_recipe(&state.pool, &payload.name, payload.ingredients).await {
        Ok(_) => (StatusCode::CREATED, "Recette créée avec succès").into_response(),
        Err(e) => {
            eprintln!("Erreur SQL Create Recipe : {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Impossible de créer la recette").into_response()
        }
    }
}
async fn get_recipe_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match logic::get_recipe_by_id(&state.pool, id).await {
        Ok(recipe) => (StatusCode::OK, Json(recipe)).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Recette introuvable").into_response(),
    }
}

async fn update_recipe_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<CreateRecipePayload>, // On réutilise le même payload que pour create
) -> impl IntoResponse {
    match logic::update_recipe(&state.pool, id, &payload.name, payload.ingredients).await {
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
    State(state): State<AppState>,
    Json(payload): Json<CreateOrderPayload>,
) -> impl IntoResponse {
    match logic::place_order_ticket(&state.pool, payload.recipe_ids).await {
        Ok(_) => {
            let _ = state.tx.send("NEW_ORDER".to_string());
            (StatusCode::CREATED, "Commande créée avec succès").into_response()
        },
        Err(e) => {
            eprintln!("Erreur lors de la commande : {}", e);
            (StatusCode::BAD_REQUEST, e).into_response()
        }
    }
}

async fn get_orders_history_handler(State(state): State<AppState>) -> impl IntoResponse {
    match logic::get_orders_history(&state.pool).await {
        Ok(history) => (StatusCode::OK, Json(history)).into_response(),
        Err(e) => {
            eprintln!("Erreur SQL Orders History : {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Erreur lors de la récupération de l'historique des commandes").into_response()
        }
    }
}

// -- HANDLERS USERS ---
// (à implémenter plus tard, pas de gestion d'utilisateurs pour l'instant)

// --- WEBSOCKET HANDLER ---

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub tx: broadcast::Sender<String>,
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, _) = socket.split();
    let mut rx = state.tx.subscribe(); // On s'abonne aux alertes

    // On boucle : dès qu'un message arrive dans state.tx, on l'envoie au client JS
    while let Ok(msg) = rx.recv().await {
        if sender.send(axum::extract::ws::Message::Text(msg)).await.is_err() {
            break; // Client déconnecté
        }
    }
}