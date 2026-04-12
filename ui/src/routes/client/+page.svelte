<script lang="ts">
	import { onMount } from 'svelte';
	import ProductNavigation from '$lib/components/client/ProductNavigation.svelte';
	import ProductCard from '$lib/components/client/ProductCard.svelte';

	let productsList: any[] = $state([]);
	let activeCategory = $state('cocktails');

	const fakeList: any[] = [
		{ id: 1, name: 'Produit 1', price: 10, image: 'path/to/image1.jpg' },
		{ id: 2, name: 'Produit 2', price: 20, image: 'path/to/image2.jpg' },
		{ id: 3, name: 'Produit 3', price: 30, image: 'path/to/image3.jpg' },
		{ id: 4, name: 'Produit 4', price: 40, image: 'path/to/image4.jpg' },
		{ id: 5, name: 'Produit 5', price: 50, image: 'path/to/image5.jpg' },
		{ id: 6, name: 'Produit 6', price: 60, image: 'path/to/image6.jpg' },
		{ id: 7, name: 'Produit 7', price: 70, image: 'path/to/image7.jpg' },
		{ id: 8, name: 'Produit 8', price: 80, image: 'path/to/image8.jpg' },
		{ id: 9, name: 'Produit 9', price: 90, image: 'path/to/image9.jpg' },
		{ id: 10, name: 'Produit 10', price: 100, image: 'path/to/image10.jpg' },
		{ id: 11, name: 'Produit 11', price: 110, image: 'path/to/image11.jpg' },
		{ id: 12, name: 'Produit 12', price: 120, image: 'path/to/image12.jpg' },
		{ id: 13, name: 'Produit 13', price: 130, image: 'path/to/image13.jpg' },
		{ id: 14, name: 'Produit 14', price: 140, image: 'path/to/image14.jpg' },
		{ id: 15, name: 'Produit 15', price: 150, image: 'path/to/image15.jpg' },
		{ id: 16, name: 'Produit 16', price: 160, image: 'path/to/image16.jpg' }
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
			<ProductNavigation onCategoryChange={(name: string) => (activeCategory = name)} />
		</div>
		<div class="products-list">
			{#each productsList as product (product.id)}
				<ProductCard name={product.name} price={product.price} image={product.image} />
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
					height: auto;
				}
			}
			.products-list {
				display: grid;
				grid-template-columns: repeat(4, 1fr);
				overflow: scroll;
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
