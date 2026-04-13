<script lang="ts">
	import { onMount } from 'svelte';
	import ProductNavigation from '$lib/components/client/ProductNavigation.svelte';
	import ProductCard from '$lib/components/client/ProductCard.svelte';
	import { gsap } from 'gsap';
	import { ScrollTrigger } from 'gsap/ScrollTrigger';


	interface Product {
    id: number;
    name: string;
    price: number;
    image: string;
}

	gsap.registerPlugin(ScrollTrigger);

	let productsList: any[] = $state([]);
	let activeCategory = $state('cocktails');

	// On utilise les chevrons < > pour définir les types possibles
let selectedProduct = $state<Product | null>(null);
	let isPopupOpen = $state(false);
	let popupRef: HTMLElement;
	let overlayRef: HTMLElement;

	const fakeList: any[] = [
		{ id: 1, name: 'Mojito', price: 10, image: 'path/to/image1.jpg' },
		{ id: 2, name: 'Sex on the beach', price: 20, image: 'path/to/image2.jpg' },
		{ id: 3, name: 'Blue Apple', price: 30, image: 'path/to/image3.jpg' },
		{ id: 4, name: 'Margarita', price: 40, image: 'path/to/image4.jpg' },
		{ id: 5, name: 'Malibu', price: 50, image: 'path/to/image5.jpg' },
		{ id: 6, name: 'Moscow Mule', price: 60, image: 'path/to/image6.jpg' },
		{ id: 7, name: 'Blue Lagoon', price: 70, image: 'path/to/image7.jpg' },
		{ id: 8, name: 'Cosmopolitan', price: 80, image: 'path/to/image8.jpg' },
		{ id: 9, name: 'Apple Jäger', price: 90, image: 'path/to/image9.jpg' },
		{ id: 10, name: 'Gin Pamplemousse', price: 100, image: 'path/to/image10.jpg' },
		{ id: 11, name: 'Pasteka', price: 110, image: 'path/to/image11.jpg' },
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

	function openPopup(product: any, clickedElement: HTMLElement) {
		selectedProduct = product;
		isPopupOpen = true;

		// On attend que Svelte crée la popup dans le DOM
		setTimeout(() => {
			// 1. POSITION DE DÉPART (First)
			const firstRect = clickedElement.getBoundingClientRect();

			// 2. POSITION D'ARRIVÉE (Last)
			// La popup est déjà centrée par le CSS, on prend sa position
			const lastRect = popupRef.getBoundingClientRect();

			// 3. CALCUL DE L'INVERSION (Invert)
			const deltaX = firstRect.left - lastRect.left;
			const deltaY = firstRect.top - lastRect.top;
			const scaleX = firstRect.width / lastRect.width;
			const scaleY = firstRect.height / lastRect.height;

			gsap.to(overlayRef, { opacity: 1, duration: 0.3 });

			// On fait "voler" la popup de la position du bouton vers le centre
			gsap.fromTo(
				popupRef,
				{
					x: deltaX,
					y: deltaY,
					scaleX: scaleX,
					scaleY: scaleY,
					opacity: 0.5,
					borderRadius: '5px' // On part du border-radius du bouton
				},
				{
					x: 0,
					y: 0,
					scaleX: 1,
					scaleY: 1,
					opacity: 1,
					borderRadius: '20px', // Border-radius final de la popup
					duration: 0.6,
					ease: 'power3.out',
					clearProps: 'transform' // Nettoie après l'anim
				}
			);
		}, 10); // Un micro-délai pour le rendu
	}

	function closePopup() {
		gsap.to(popupRef, {
			opacity: 0,
			scale: 0.8,
			duration: 0.3,
			ease: 'power2.in',
			onComplete: () => {
				isPopupOpen = false;
				selectedProduct = null;
			}
		});
		gsap.to(overlayRef, { opacity: 0, duration: 0.3 });
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
				<ProductCard
					name={product.name}
					price={product.price}
					image={product.image}
					openAction={(clickedElement: any) => openPopup(product, clickedElement)}
				/>
			{/each}
			{#if isPopupOpen && selectedProduct}
				<div class="popup-overlay" bind:this={overlayRef} onclick={closePopup}></div>

				<div class="product-popup" bind:this={popupRef}>
					<button class="close-btn" onclick={closePopup}>×</button>

					<div class="popup-content">
						<img src={selectedProduct.image} alt={selectedProduct.name} />
						<h2>{selectedProduct.name}</h2>
						<p class="price">{selectedProduct.price} €</p>
						<p class="description">Une délicieuse description de ton cocktail préféré...</p>
						<button class="add-btn">Ajouter au panier</button>
					</div>
				</div>
			{/if}
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
			background-color: white;
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
				gap: 15px;
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
