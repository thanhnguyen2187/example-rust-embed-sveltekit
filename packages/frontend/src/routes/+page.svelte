<script lang="ts">
async function fetchData() {
  const resp = await fetch("/api/data");
  return await resp.text();
}
</script>

<div class="prose max-w-none">
    <h1>Example Rust Embed SvelteKit Project</h1>

    <p>
        {#await fetchData()}
            Loading... (should take precisely 3 seconds)
        {:then data}
            {data}
        {/await}
    </p>

    <a href="child-url">Going to another route</a>
</div>
