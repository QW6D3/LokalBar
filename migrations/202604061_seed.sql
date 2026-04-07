-- 1. Ajout des items dans le catalogue
INSERT INTO items (name, category, unit) VALUES 
('Vodka', 'Alcool', 'ml'),
('Liqueur de Pêche', 'Alcool', 'ml'),
('Jus d''Orange', 'Soft', 'ml'),
('Jus de Cranberry', 'Soft', 'ml'),
('Rhum Blanc', 'Alcool', 'ml'),
('Eau Gazeuse', 'Soft', 'ml'),
('Sucre de Canne', 'Sirop', 'ml'),
('Menthe Fraîche', 'Garniture', 'feuille'),
('Citron Vert', 'Fruit', 'quartier');

-- 2. Remplissage du stock réel (pour que place_order_ticket ne renvoie pas d'erreur)
-- On lie les items aux stocks via leur item_id
INSERT INTO stock (item_id, brand_name, current_quantity, total_capacity, buy_price_cents, is_open) VALUES 
(1, 'Absolut', 1000, 1000, 2000, 1),      -- Vodka
(2, 'Pecher Mignon', 700, 700, 1500, 1),  -- Liqueur de Pêche
(3, 'Granini', 2000, 2000, 500, 1),       -- Jus Orange
(4, 'Ocean Spray', 2000, 2000, 600, 1),   -- Jus Cranberry
(5, 'Havana Club', 1000, 1000, 2200, 1),  -- Rhum
(6, 'Perrier', 1500, 1500, 300, 1),       -- Eau Gazeuse
(7, 'Canadou', 500, 500, 400, 1);         -- Sucre

-- 3. Création des Recettes
INSERT INTO recipes (name, price_cents) VALUES 
('Sex on the Beach', 850),
('Mojito', 900);

-- 4. Liaison Ingrédients -> Recettes
-- Sex on the Beach (ID 1)
INSERT INTO recipe_ingredients (recipe_id, item_id, amount) VALUES 
(1, 1, 40),  -- 40ml Vodka
(1, 2, 20),  -- 20ml Pêche
(1, 3, 60),  -- 60ml Orange
(1, 4, 60);  -- 60ml Cranberry

-- Mojito (ID 2)
INSERT INTO recipe_ingredients (recipe_id, item_id, amount) VALUES 
(2, 5, 50),  -- 50ml Rhum
(2, 7, 20),  -- 20ml Sucre
(2, 6, 100); -- 100ml Eau Gazeuse