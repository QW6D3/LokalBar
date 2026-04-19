<script lang="ts">
	import { onMount, tick } from 'svelte';
	import { mount, unmount } from 'svelte';
	import ProductNavigation from '$lib/components/client/ProductNavigation.svelte';
	import ProductInformations from './../../lib/components/client/ProductInformations.svelte';
	import ProductCard from '$lib/components/client/ProductCard.svelte';
	import { gsap } from 'gsap';
	import { ScrollTrigger } from 'gsap/ScrollTrigger';

	gsap.registerPlugin(ScrollTrigger);


	interface Product {
		id: number;
		name: string;
		price: number;
		image: string;
	}

	interface CartItem {
		product: Product;
		quantity: number;
	}

	let productsList: any[] = $state([]);
	let activeCategory = $state('cocktails');
	let clonedCard: HTMLElement | null = null;
	let originalButton: HTMLElement | null = null;
	let isExpanded = $state(false);
	let selectedProduct = $state<Product | null>(null);
	let overlayRef: HTMLElement;
	let mountedComponent: ReturnType<typeof mount> | null = null;
	let cart: Array<CartItem> = $state([]);

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

    clonedCard = document.createElement('div');
    clonedCard.style.cssText = `
        position: fixed;
        top: ${rect.top}px;
        left: ${rect.left}px;
        width: ${rect.width}px;
        height: ${rect.height}px;
        z-index: 1001;
        border-radius: 5px;
        overflow: hidden;
        background-color: white; 
    `;

    originalButton.style.opacity = '0';
    document.body.appendChild(clonedCard);

    gsap.to(overlayRef, { opacity: 1, display: 'block', duration: 0.2 });

    const targetW = 400;
    const targetH = 600;

    gsap.to(clonedCard, {
        top: (window.innerHeight - targetH) / 2,
        left: (window.innerWidth - targetW) / 2,
        width: targetW,
        height: targetH,
        borderRadius: '20px',
        duration: 0.9,
        ease: 'power4.out',
        onStart: () => {
            mountedComponent = mount(ProductInformations, {
                target: clonedCard!,
                props: {
                    product: selectedProduct,
                    onClose: closeCard,
                    onAddToCart: (p: Product) => console.log('Ajouté :', p)
                }
            });
        }
    });
}
	function closeCard() {
		if (!clonedCard || !originalButton) return;

		isExpanded = false;

		if (mountedComponent) {
			unmount(mountedComponent);
			mountedComponent = null;
		}

		clonedCard.innerHTML = 'i';
		clonedCard.style.display = 'flex';
		clonedCard.style.alignItems = 'center';
		clonedCard.style.justifyContent = 'center';
		clonedCard.style.color = 'white';
		clonedCard.style.background = 'blue';
		clonedCard.style.fontSize = '1.2rem';

		const rect = originalButton.getBoundingClientRect();

		gsap.to(overlayRef, {
			opacity: 0,
			duration: 0.3,
			onComplete: () => {
				overlayRef.style.display = 'none';
			}
		});

		gsap.to(clonedCard, {
			top: rect.top,
			left: rect.left,
			width: rect.width,
			height: rect.height,
			borderRadius: '5px',
			duration: 0.5,
			ease: 'power3.in',
			onComplete: () => {
				clonedCard?.remove();
				clonedCard = null;

				if (originalButton) {
					originalButton.style.opacity = '1';
					originalButton.style.pointerEvents = 'auto';
				}
				originalButton = null;
				selectedProduct = null;

				document.body.style.overflow = '';
			}
		});
	}
	function addProductToCart(product: Product) {
    const existingItem = cart.find(item => item.product.id === product.id);

    if (existingItem) {
        existingItem.quantity += 1;
    } else {
        cart.push({ 
            product: { ...product }, 
            quantity: 1 
        });
    }
}

	onMount(async () => {
		getProducts();

		await tick();

		const cards = document.querySelectorAll('.product-card > *');
		gsap.to(cards, {
			opacity: 1,
			y: 0,
			duration: 0.8,
			stagger: 0.1,
			ease: 'power4.out',
			scrollTrigger: {
				trigger: '.products-list',
				start: 'top 80%',
				end: 'bottom 20%',
				toggleActions: 'play none none reverse'
			}
		})
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
					addToCart={() => addProductToCart(product)}
				/>
			{/each}
		</div>
	</section>

	<section class="basket">
		<div>
			<h2>Panier</h2>
			{#each cart as item}
				<p>{item.product.name} - Quantité: {item.quantity}</p>
			{/each}
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
