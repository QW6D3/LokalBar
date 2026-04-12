<script lang="ts">
    interface Props {
        name: string;
        price: number;
        image: string;
    }

    let { name, price, image }: Props = $props();
</script>

<div class="product-card">
    <button class="product-recipe">i</button>
    <img src={image} alt={name} class="product-image" />
    <div class="product-infos">
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
        width: 100%;
        background-color: white;
}
</style>