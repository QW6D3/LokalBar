<script lang="ts">
	import { onMount, tick } from 'svelte';
	import { gsap } from 'gsap';

    let { onCategoryChange } = $props();
	let navigationItems: { id: number; name: string; label: string }[] = $state([]);
    let activeIndex = $state(0);
	let barRef: HTMLElement;
	let navRef: HTMLElement;
	

	onMount(async () => {

        navigationItems = await getNavigationsItems();

        await tick(); 

        const firstBtn = navRef.querySelector('.nav-item') as HTMLElement;
        if (firstBtn) {
            gsap.set(barRef, {
                x: firstBtn.offsetLeft,
                width: firstBtn.offsetWidth
            });
        }
    });

	async function getNavigationsItems() {
		// A remplacer par un fetch une fois que la la logique du back rust correspond
		return [
			{ id: 1, name: 'cocktails', label: 'Cocktails' },
			{ id: 2, name: 'mocktails', label: 'Mocktails' },
			{ id: 3, name: 'beers', label: 'Bières' },
			{ id: 4, name: 'softs', label: 'Softs' }
		];
	}

	function moveBar(target: HTMLElement, index: number, category: string) {

        activeIndex = index;
		gsap.to(barRef, {
			x: target.offsetLeft,
			width: target.offsetWidth,
			duration: 0.8,
			ease: 'elastic.out(1, 1)',
			overwrite: 'auto'
		});

        if (onCategoryChange) {
            onCategoryChange(category);
        }
	}
</script>

<nav bind:this={navRef}>
	{#each navigationItems as item, i (item.id)}
		<button class="nav-item" onclick={(e) => moveBar(e.currentTarget, i, item.name)}>
			{item.label}
		</button>
	{/each}
	<div class="bar" bind:this={barRef}></div>
</nav>

<style lang="scss">
	nav {
		position: relative;
		display: flex;
		gap: 20px;
		padding: 10px 0;

		.nav-item {
			background: none;
			border: none;
			font-size: 1.2rem;
			cursor: pointer;
			padding: 4px 16px;
			color: black;
            -webkit-tap-highlight-color: transparent;
		}

		.bar {
			position: absolute;
			bottom: 0;
			left: 0;
			height: 2px;
			width: 0;
			background: black;
			border-radius: 2px;
		}
	}
</style>
