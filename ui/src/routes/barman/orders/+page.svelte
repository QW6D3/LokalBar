<script>
	import { onMount } from 'svelte';

	let orders = $state([]);
	let socket;
	let selectedRecipeId = '';

	// Fonction pour récupérer les commandes
	async function getOrders() {
		try {
			const res = await fetch('http://localhost:8080/api/orders');
			if (res.ok) {
				const data = await res.json();

				orders = data; // ✅ Remplacer au lieu de push
				console.log("receivedData = ", orders); // ✅ Log après affectation
			} else {
				console.error('Erreur chargement:', res.statusText);
			}
		} catch (err) {
			console.error('Erreur réseau:', err);
		}
	}

	// Créer une commande
	async function placeOrder() {
		if (!selectedRecipeId) return alert('Choisis un ID de recette !');

		const payload = {
			recipe_ids: [parseInt(selectedRecipeId)]
		};

		const res = await fetch('http://localhost:8080/api/orders', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify(payload)
		});

		if (res.ok) {
			console.log('Commande envoyée ! Attente du WebSocket...');
			selectedRecipeId = '';
		} else {
			const error = await res.text();
			alert('Erreur : ' + error);
		}
	}

	onMount(() => {
		getOrders();

		// Connexion WebSocket
		socket = new WebSocket('ws://localhost:8080/ws');

		socket.onmessage = (event) => {
			if (event.data === 'NEW_ORDER') {
				console.log('🔔 Nouvelle commande détectée via WS !');
				getOrders();
			}
		};

		return () => socket?.close();
	});
</script>

<main>
	<h1>LokalBar Dashboard</h1>

	<section class="test-panel">
		<h3>Simuler une commande</h3>
		<input type="number" bind:value={selectedRecipeId} placeholder="ID de la recette (ex: 1)" />
		<button on:click={placeOrder}>Envoyer la commande</button>
	</section>

	<hr />

	<section>
		<h3>Commandes en cours</h3>
		{#if orders.length === 0}
			<p>Aucune commande.</p>
		{:else}
			<div class="grid">
				{#each orders as order}
					<div class="card">
						<strong>Commande #{order.id}</strong>
						<p>Cocktails : {order.recipes?.join(', ')}</p>
						<small>Statut : {order.status}</small>
					</div>
				{/each}
			</div>
		{/if}
	</section>
</main>

<style>
	.test-panel {
		background: #f4f4f4;
		padding: 1rem;
		border-radius: 8px;
	}
	.grid {
		display: flex;
		gap: 10px;
		flex-wrap: wrap;
	}
	.card {
		border: 1px solid #ccc;
		padding: 10px;
		border-radius: 5px;
		min-width: 150px;
	}
</style>