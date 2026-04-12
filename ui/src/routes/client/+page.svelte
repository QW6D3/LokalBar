<script lang="ts">
	import { onMount } from 'svelte';
	import ProductNavigation from '$lib/components/client/ProductNavigation.svelte';

	let productsList: any[] = $state([]);
	let activeCategory = $state('cocktails');

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
	<section class="products-container">
		<div class="header-list">
			<img class="logo" src="path/to/image.jpg" alt="LokalBar" />
			<ProductNavigation onCategoryChange={(name: string) => activeCategory = name}/>
		</div>
		<div class="products-list">
		<p>La catégorie dans le parent est : {activeCategory}</p>
			{#each productsList as product (product.id)}
				<div class="product">
					<img src={product.image} alt={product.name} />
					<div>
						<p>{product.name}</p>
					</div>
				</div>
			{/each}
		</div>
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
		.products-container {
			box-sizing: border-box;
			height: 100%;
			width: 70%;
			background-color: lightcoral;
			display: flex;
			flex-direction: column;
			gap: 1rem;
			padding: 1rem;
			.header-list {
				display: flex;
				align-items: center;
				gap: 2rem;
				.logo {
					width: 150px;
					height: auto;
				}
			}
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
