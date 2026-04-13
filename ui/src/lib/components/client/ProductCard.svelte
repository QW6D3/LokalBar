<script lang="ts">
	interface Props {
		name: string;
		price: number;
		image: string;
		openAction: (clickedElement: HTMLElement) => void;
	}

	let { name, price, image, openAction }: Props = $props();

	let infoButtonRef: HTMLElement;
</script>

<div class="product-card">
	<button
		class="product-details"
		bind:this={infoButtonRef}
		onclick={() => openAction(infoButtonRef)}>i</button
	>
	<img src={image} alt={name} class="product-image" />
	<div class="product-header">
		<h3>{name}</h3>
		<button class="add-to-cart">
			{price} €
		</button>
	</div>
</div>

<style lang="scss">
	.product-card {
		display: flex;
		flex-direction: column;
		position: relative;
		height: 250px;
		width: auto;
		border: solid 1px #ccc;
		.product-details {
			position: absolute;
			top: 8px;
			right: 8px;
			background: blue;
			color: white;
			width: 32px;
			height: 32px;
			border: none;
			font-size: 1.2rem;
			cursor: pointer;
			border-radius: 5px;
		}
		img {
			height: 100%;
		}
		.product-header {
			font-family: 'Paquito';
			height: 50px;
			display: flex;
			justify-content: space-between;
			align-items: center;
			padding: 0 5px;
			button {
				width: 80px;
				height: 40px;
			}
		}
	}
</style>
