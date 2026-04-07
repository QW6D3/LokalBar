use crate::models;
use sqlx::SqlitePool;

// --- STOCK ---
pub async fn get_full_stock(pool: &SqlitePool) -> Result<Vec<models::StockEntry>, sqlx::Error> {
    sqlx::query_as!(
        models::StockEntry,
        r#"
        SELECT 
            s.id as "id!", 
            i.name as "name!", 
            s.brand_name, 
            s.current_quantity as "current_quantity!", 
            i.unit as "unit!", 
            s.is_open as "is_open!: bool"
        FROM stock s
        JOIN items i ON s.item_id = i.id
        "#
    )
    .fetch_all(pool)
    .await
}

pub async fn add_product_to_stock(
    pool: &SqlitePool,
    name: &str,
    category: &str,
    brand: &str,
    amount: f64,
    unit: &str,
    price_cents: i32,
) -> Result<(), sqlx::Error> {
    let item = sqlx::query!(
        "INSERT INTO items (name, category, unit) VALUES (?, ?, ?) 
         ON CONFLICT(name) DO UPDATE SET name=name RETURNING id",
        name,
        category,
        unit
    )
    .fetch_one(pool)
    .await?;

    sqlx::query!(
        "INSERT INTO stock (item_id, brand_name, current_quantity, total_capacity, buy_price_cents) 
         VALUES (?, ?, ?, ?, ?)",
        item.id, brand, amount, amount, price_cents
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_product_from_stock(
    pool: &SqlitePool,
    stock_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM stock WHERE id = ?", stock_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn reduce_stock(
    pool: &SqlitePool,
    stock_id: i64,
    amount_to_remove: f64,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE stock SET current_quantity = current_quantity - ? WHERE id = ?",
        amount_to_remove,
        stock_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

// --- RECIPES ---

pub async fn create_recipe(
    pool: &SqlitePool,
    name: &str,
    ingredients: Vec<(i64, f64)>,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    let recipe = sqlx::query!("INSERT INTO recipes (name) VALUES (?) RETURNING id", name)
        .fetch_one(&mut *tx)
        .await?;

    for (item_id, amount) in ingredients {
        sqlx::query!(
            "INSERT INTO recipe_ingredients (recipe_id, item_id, amount) VALUES (?, ?, ?)",
            recipe.id,
            item_id,
            amount
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}

pub async fn get_all_recipes(
    pool: &SqlitePool,
) -> Result<Vec<models::RecipeWithIngredients>, sqlx::Error> {
    let recipes = sqlx::query!("SELECT id, name FROM recipes")
        .fetch_all(pool)
        .await?;

    let mut full_recipes = Vec::new();

    for recipe in recipes {
        let ingredients = sqlx::query_as!(
            models::IngredientDetail,
            r#"
            SELECT i.name as "item_name!", ri.amount as "amount!", i.unit as "unit!"
            FROM recipe_ingredients ri
            JOIN items i ON ri.item_id = i.id
            WHERE ri.recipe_id = ?
            "#,
            recipe.id
        )
        .fetch_all(pool)
        .await?;

        full_recipes.push(models::RecipeWithIngredients {
            id: recipe.id,
            name: recipe.name,
            ingredients,
        });
    }

    Ok(full_recipes)
}

pub async fn get_recipe_by_id(
    pool: &SqlitePool,
    recipe_id: i64,
) -> Result<models::RecipeWithIngredients, sqlx::Error> {
    let recipe = sqlx::query!("SELECT id, name FROM recipes WHERE id = ?", recipe_id)
        .fetch_one(pool)
        .await?;

    let ingredients = sqlx::query_as!(
        models::IngredientDetail,
        r#"
        SELECT i.name as "item_name!", ri.amount as "amount!", i.unit as "unit!"
        FROM recipe_ingredients ri
        JOIN items i ON ri.item_id = i.id
        WHERE ri.recipe_id = ?
        "#,
        recipe_id
    )
    .fetch_all(pool)
    .await?;

    Ok(models::RecipeWithIngredients {
        id: recipe.id,
        name: recipe.name,
        ingredients,
    })
}

pub async fn update_recipe(
    pool: &SqlitePool,
    recipe_id: i64,
    new_name: &str,
    updated_ingredients: Vec<(i64, f64)>,
) -> Result<models::RecipeWithIngredients, sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query!(
        "UPDATE recipes SET name = ? WHERE id = ?",
        new_name,
        recipe_id
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query!(
        "DELETE FROM recipe_ingredients WHERE recipe_id = ?",
        recipe_id
    )
    .execute(&mut *tx)
    .await?;

    for (item_id, amount) in updated_ingredients {
        sqlx::query!(
            "INSERT INTO recipe_ingredients (recipe_id, item_id, amount) VALUES (?, ?, ?)",
            recipe_id,
            item_id,
            amount
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    get_recipe_by_id(pool, recipe_id).await
}

// --- ORDERS ---

pub async fn place_order_ticket(pool: &SqlitePool, recipe_ids: Vec<i64>) -> Result<i64, String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    let order = sqlx::query!("INSERT INTO orders (status) VALUES ('COMPLETED') RETURNING id")
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    let order_id = order.id;

    for rid in recipe_ids {
        let ingredients = sqlx::query!(
            r#"
            SELECT i.name as "name!", ri.amount as "amount!"
            FROM recipe_ingredients ri
            JOIN items i ON ri.item_id = i.id
            WHERE ri.recipe_id = ?
            "#,
            rid
        )
        .fetch_all(&mut *tx)
        .await
        .map_err(|_| "Une des recettes est introuvable".to_string())?;

        for ing in ingredients {
            let res = sqlx::query!(
                r#"
                UPDATE stock 
                SET current_quantity = current_quantity - ? 
                WHERE item_id = (SELECT id FROM items WHERE name = ?) 
                AND current_quantity >= ?
                "#,
                ing.amount,
                ing.name,
                ing.amount
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

            if res.rows_affected() == 0 {
                return Err(format!(
                    "Stock insuffisant pour {} dans une des recettes",
                    ing.name
                ));
            }
        }

        sqlx::query!(
            "INSERT INTO order_items (order_id, recipe_id) VALUES (?, ?)",
            order_id,
            rid
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    }

    tx.commit().await.map_err(|e| e.to_string())?;

    Ok(order_id)
}

pub async fn get_orders_history(
    pool: &SqlitePool,
) -> Result<Vec<models::OrderWithItems>, sqlx::Error> {
    let orders = sqlx::query!(
        r#"
    SELECT id, status, CAST(created_at AS TEXT) as "created_at!" 
    FROM orders 
    ORDER BY created_at DESC
    "#
    )
    .fetch_all(pool)
    .await?;

    let mut full_history = Vec::new();

    for order in orders {
        let items = sqlx::query!(
            r#"
            SELECT r.name as "recipe_name!"
            FROM order_items oi
            JOIN recipes r ON oi.recipe_id = r.id
            WHERE oi.order_id = ?
            "#,
            order.id
        )
        .fetch_all(pool)
        .await?;

        let recipe_names = items.into_iter().map(|i| i.recipe_name).collect();

        full_history.push(models::OrderWithItems {
            id: order.id,
            status: order.status,
            recipes: recipe_names,
            created_at: order.created_at
        });
    }

    Ok(full_history)
}


// --- Users ---


pub async fn create_user(){

}
pub async fn get_user(user_id: i64){

}
pub async fn update_user(user_id: i64){

}
pub async fn delete_user(user_id: i64){

}
pub async fn list_users(){

}
pub async fn add_credits_to_user(user_id: i64, amount: i64){

}
pub async fn remove_credits_from_user(user_id: i64, amount: i64){

}