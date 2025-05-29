<script lang="ts">
	import '../app.css';
	
	let { children } = $props();
</script>

<main class="p-4 container">
    {@render children()}
</main>
