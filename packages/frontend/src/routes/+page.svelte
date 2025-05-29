<script lang="ts">
async function fetchData() {
  const resp = await fetch("/api/data");
  return await resp.text();
}
</script>

<div class="prose max-w-none">
    <h1>SPA Frontend</h1>

    <p>
        {#await fetchData()}
            Fetching data from <code>/api/data</code> <br/>
            (should take precisely 3 seconds)
        {:then data}
            {data}
        {/await}
    </p>

    <a href="child-url">Go to another route</a>
</div>
