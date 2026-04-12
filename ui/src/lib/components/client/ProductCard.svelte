<script lang="ts">
    import openPopUp from '$lib/components/client/OpenPop-up.svelte';

    interface Props {
        name: string;
        price: number;
        image: string;
    }

    let { name, price, image }: Props = $props();
</script>

<div class="product-card">
    <button onclick={openPopUp}>i</button>
    <img src={image} alt={name} class="product-image" />
    <div>
        <h3>{name}</h3>
        <button class="add-to-cart">
            {price} €
        </button>
    </div>
</div>

<style>
    .add-to-cart {
        cursor: pointer;
    }
</style>