<script lang="ts">
	import { onMount } from 'svelte';
	import AddToCartButton from '$lib/components/AddToCartButton.svelte';

	let productsList: any[] = $state([]);

	const fakeList: any[] = [
		{ id: 1, name: 'Produit 1', price: 10, image: 'path/to/image1.jpg' },
		{ id: 2, name: 'Produit 2', price: 20, image: 'path/to/image2.jpg' },
		{ id: 3, name: 'Produit 3', price: 30, image: 'path/to/image3.jpg' }
	];

	productsList = fakeList;



	let cart: { productId: number; quantity: number }[] = [];

	async function getProducts() {
		try {
			const res = await fetch('http://localhost:8080/api/products');
			if (res.ok) {
				const data = await res.json();
				productsList = data;
			} else {
				console.error('Erreur chargement:', res.statusText);
			}
		} catch (err) {
			console.error('Erreur réseau:', err);
		}
	}
	onMount(() => {
		getProducts();
	});
</script>

<main>
	<img class="logo" src="path/to/image.jpg" alt="LokalBar" />
	<section class="products">
		{#each productsList as product (product.id)}
			<div class="product">
				<img src={product.image} alt={product.name}>
				<div>
					<p>{product.name}</p>
					<AddToCartButton productId={product.id} productPrice={product.price}/>
				</div>
			</div>
		{/each}
	</section>
	<section class="basket">
		<div>
			<h2>Panier</h2>
			<ul>
				<li>Produit 1 - Quantité: 2</li>
				<li>Produit 2 - Quantité: 1</li>
			</ul>
		</div>
		<button>Valider la commande</button>
	</section>
</main>

<style lang="scss">
	main {
		position: relative;
		display: flex;
		flex-direction: row;
		height: 100vh;
		.logo {
			position: absolute;
			top: 2rem;
			left: 2rem;
		}
		.products {
			box-sizing: border-box;
			height: 100%;
			width: 70%;
			background-color: lightcoral;
			display: grid;
			grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
			gap: 1rem;
			padding: 1rem;
		}
		.basket {
			display: flex;
			box-sizing: border-box;
			flex-direction: column;
			height: 100%;
			width: 30%;
			background-color: lightskyblue;
			justify-content: space-between;
			padding: 1rem;
		}
	}
</style>