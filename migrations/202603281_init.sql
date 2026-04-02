-- 1. Le catalogue des produits
CREATE TABLE IF NOT EXISTS items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE, 
    category TEXT,
    unit TEXT NOT NULL DEFAULT 'ml'
);

-- 2. Les stocks réels
CREATE TABLE IF NOT EXISTS stock (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    item_id INTEGER REFERENCES items(id),
    brand_name TEXT,           
    current_quantity REAL,    
    total_capacity REAL,      
    buy_price_cents INTEGER,  
    is_open BOOLEAN DEFAULT 0 
);

-- 3. Les recettes de cocktails
CREATE TABLE IF NOT EXISTS recipes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    price_cents INTEGER DEFAULT 0
);

-- 4. Les ingrédients par recette
CREATE TABLE IF NOT EXISTS recipe_ingredients (
    recipe_id INTEGER REFERENCES recipes(id),
    item_id INTEGER REFERENCES items(id),
    amount REAL NOT NULL,
    PRIMARY KEY (recipe_id, item_id)
);

-- 5. L'historique des tickets
CREATE TABLE IF NOT EXISTS orders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    status TEXT NOT NULL DEFAULT 'COMPLETED',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 6. le détail des tickets
CREATE TABLE IF NOT EXISTS order_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    order_id INTEGER REFERENCES orders(id),
    recipe_id INTEGER REFERENCES recipes(id)
);