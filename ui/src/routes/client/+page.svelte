<script lang="ts">
	import { onMount } from 'svelte';
	import ProductNavigation from '$lib/components/client/ProductNavigation.svelte';
	import ProductCard from '$lib/components/client/ProductCard.svelte';
	import { gsap } from 'gsap';

	interface Product {
		id: number;
		name: string;
		price: number;
		image: string;
	}

	let productsList: any[] = $state([]);
	let activeCategory = $state('cocktails');
	let clonedCard: HTMLElement | null = null;
	let originalButton: HTMLElement | null = null;
	let isExpanded = $state(false);
	let selectedProduct = $state<Product | null>(null);
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

	function openCard(clickedButton: HTMLElement, product: any) {
		selectedProduct = product;
		originalButton = clickedButton;

		const rect = clickedButton.getBoundingClientRect();

		// Crée le clone du bouton
		clonedCard = clickedButton.cloneNode(true) as HTMLElement;

		// Positionne le clone exactement sur le bouton original
		clonedCard.style.position = 'fixed';
		clonedCard.style.top = `${rect.top}px`;
		clonedCard.style.left = `${rect.left}px`;
		clonedCard.style.width = `${rect.width}px`;
		clonedCard.style.height = `${rect.height}px`;
		clonedCard.style.margin = '0';
		clonedCard.style.zIndex = '1001';
		clonedCard.style.borderRadius = '5px';
		clonedCard.style.overflow = 'hidden';

		// Cache le bouton original
		originalButton.style.opacity = '0';
		originalButton.style.pointerEvents = 'none';

		// Bloque le scroll
		document.body.style.overflow = 'hidden';

		// Ajoute le clone au body
		document.body.appendChild(clonedCard);

		// Anime l'overlay
		gsap.fromTo(overlayRef, { opacity: 0, display: 'block' }, { opacity: 1, duration: 0.3 });

		// Taille cible de la carte étendue
		const targetW = 400;
		const targetH = 600;

		// Anime le clone vers le centre
		gsap.to(clonedCard, {
			top: (window.innerHeight - targetH) / 2,
			left: (window.innerWidth - targetW) / 2,
			width: targetW,
			height: targetH,
			borderRadius: '20px',
			backgroundColor: 'white',
			duration: 0.5,
			ease: 'power3.out',
			onComplete: () => {
				// Une fois l'animation terminée, remplace le contenu du clone
				isExpanded = true;
				renderExpandedContent();
			}
		});
	}

	function renderExpandedContent() {
		if (!clonedCard || !selectedProduct) return;

		// Remplace le contenu du clone par le vrai contenu
		clonedCard.innerHTML = `
			<div style="
				display: flex;
				flex-direction: column;
				height: 100%;
				padding: 1.5rem;
				box-sizing: border-box;
				font-family: sans-serif;
			">
				<button id="close-btn" style="
					align-self: flex-end;
					background: none;
					border: none;
					font-size: 1.5rem;
					cursor: pointer;
					line-height: 1;
					color: #333;
				">×</button>
				<h2 style="margin: 0.5rem 0;">${selectedProduct.name}</h2>
				<p style="font-size: 1.2rem; font-weight: 600; color: #333;">${selectedProduct.price} €</p>
				<p style="color: #666; font-size: 0.9rem;">Une délicieuse description de ton cocktail préféré...</p>
				<button style="
					margin-top: auto;
					background: #222;
					color: white;
					border: none;
					padding: 0.75rem 1.5rem;
					border-radius: 10px;
					cursor: pointer;
					font-size: 1rem;
				">Ajouter au panier</button>
			</div>
		`;

		// Attache l'event du bouton fermer
		clonedCard.querySelector('#close-btn')?.addEventListener('click', closeCard);
	}

	function closeCard() {
		if (!clonedCard || !originalButton) return;

		isExpanded = false;

		const rect = originalButton.getBoundingClientRect();

		// Vide le contenu étendu et remet le "i"
		clonedCard.innerHTML = 'i';
		clonedCard.style.display = 'flex';
		clonedCard.style.alignItems = 'center';
		clonedCard.style.justifyContent = 'center';
		clonedCard.style.color = 'white';
		clonedCard.style.background = 'blue';
		clonedCard.style.fontSize = '1.2rem';

		// Anime l'overlay
		gsap.to(overlayRef, { opacity: 0, duration: 0.3, onComplete: () => {
			overlayRef.style.display = 'none';
		}});

		// Anime le clone vers la position du bouton original
		gsap.to(clonedCard, {
			top: rect.top,
			left: rect.left,
			width: rect.width,
			height: rect.height,
			borderRadius: '5px',
			duration: 0.4,
			ease: 'power3.in',
			onComplete: () => {
				// Nettoie tout
				clonedCard?.remove();
				clonedCard = null;

				if (originalButton) {
					originalButton.style.opacity = '1';
					originalButton.style.pointerEvents = 'auto';
				}
				originalButton = null;
				selectedProduct = null;

				// Débloque le scroll
				document.body.style.overflow = '';
			}
		});
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
					openAction={(clickedButton: HTMLElement) => openCard(clickedButton, product)}
				/>
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

	<!-- Overlay -->
	<div class="popup-overlay" bind:this={overlayRef} onclick={closeCard}></div>
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

	.popup-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		opacity: 0;
		display: none;
		z-index: 1000;
		cursor: pointer;
	}
</style>