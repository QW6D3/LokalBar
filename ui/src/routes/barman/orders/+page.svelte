<!-- Dimension fixe [1920x420] -->
<script>
let fakeOrders = [
    {
        id: 1,
        recipes: ["Mojito", "Caipirinha"],
        created_at: "2024-06-01T12:00:00Z"
    },
    {
        id: 2,
        recipes: ["Margarita"],
        created_at: "2024-06-01T12:05:00Z"
    },
    {
        id: 3,
        recipes: ["Old Fashioned"],
        created_at: "2024-06-01T12:10:00Z"
    }
];
let orders = [...fakeOrders];
</script>
<div>
    <h1>Orders</h1>
    <div class="orderlist"></div>
    {#each orders}
</div>
<style>

</style>