<script lang="ts">
	interface Product {
		id: number;
		name: string;
		price: number;
		image: string;
	}

	let {
		product,
		onClose,
		onAddToCart
	}: {
		product: Product;
		onClose: () => void;
		onAddToCart: (product: Product) => void;
	} = $props();
</script>

<div class="expanded-card">
	<button class="close-btn" onclick={onClose}>×</button>

	<div class="content">
		<img src={product.image} alt={product.name} class="product-image" />
		<h2>{product.name}</h2>
		<p class="price">{product.price} €</p>
		<p class="description">Une délicieuse description de ton cocktail préféré...</p>
	</div>

	<button class="add-to-cart-btn" onclick={() => onAddToCart(product)}>
		Ajouter au panier
	</button>
</div>

<style lang="scss">
	.expanded-card {
		display: flex;
		flex-direction: column;
		height: 100%;
		padding: 1.5rem;
		box-sizing: border-box;
		font-family: sans-serif;
	}

	.close-btn {
		align-self: flex-end;
		background: none;
		border: none;
		font-size: 1.5rem;
		cursor: pointer;
		line-height: 1;
		color: #333;
		padding: 0;
	}

	.content {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.product-image {
		width: 100%;
		height: 200px;
		object-fit: cover;
		border-radius: 10px;
	}

	h2 {
		margin: 0.5rem 0 0;
		font-size: 1.4rem;
	}

	.price {
		font-size: 1.2rem;
		font-weight: 600;
		color: #333;
		margin: 0;
	}

	.description {
		color: #666;
		font-size: 0.9rem;
		margin: 0;
	}

	.add-to-cart-btn {
		margin-top: auto;
		background: #222;
		color: white;
		border: none;
		padding: 0.75rem 1.5rem;
		border-radius: 10px;
		cursor: pointer;
		font-size: 1rem;

		&:hover {
			background: #444;
		}
	}
</style>