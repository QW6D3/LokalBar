use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StockEntry {
    pub id: i64,
    pub name: String,
    pub brand_name: Option<String>,
    pub current_quantity: f64,
    pub unit: String,
    pub is_open: bool,
}

#[derive(Serialize, Deserialize)]
pub struct RecipeWithIngredients {
    pub id: i64,
    pub name: String,
    pub ingredients: Vec<IngredientDetail>,
}

#[derive(Serialize, Deserialize)]
pub struct IngredientDetail {
    pub item_name: String,
    pub amount: f64,
    pub unit: String,
}

#[derive(Serialize, Deserialize)]
pub struct OrderWithItems {
    pub id: i64,
    pub status: String,
    pub recipes: Vec<String>,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub image_url: Option<String>,
    pub credits: i64,
    pub fav_recipes: Vec<RecipeWithIngredients>,
}