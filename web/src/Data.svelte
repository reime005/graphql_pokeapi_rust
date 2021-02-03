<script lang="ts">
  import { initClient } from "@urql/svelte";
  import { operationStore, query } from "@urql/svelte";

  let offset = 0;
  let limit = 10;

  const pokemons = operationStore(
    `
    query($offset: Int!, $limit: Int!) {
      pokemons(offset: $offset, limit: $limit) {
        count
        results {
          name
        }
      }
    }
  `,
    { offset, limit },
    { requestPolicy: "network-only" }
  );
  query(pokemons);
</script>

{#if $pokemons.fetching}
  <p>Loading...</p>
{:else if $pokemons.error}
  <p>Oh no... {$pokemons.error.message}</p>
{:else}
  <ul>
    {#each pokemons.data.pokemons.results as d (d.name)}
      <li>{d.name}</li>
    {/each}
  </ul>
{/if}

<style>
</style>
