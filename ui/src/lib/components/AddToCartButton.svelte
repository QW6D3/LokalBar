<script lang=ts>

interface Props {
    productId: number;
    productPrice: number;
}
let { productId, productPrice }: Props = $props();

</script>

<button>
	{productPrice}
</button>

<style>
button {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background-color: #007bff;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
}
</style>