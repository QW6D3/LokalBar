<script>
	import { onMount } from 'svelte';

	async function getProducts() {
		try {
			const res = await fetch('http://localhost:8080/api/products');
			if (res.ok) {
				const data = await res.json();
				console.log('receivedData = ', data);
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
	<section class="products"></section>
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
			height: 100%;
			width: 70%;
			background-color: lightcoral;
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