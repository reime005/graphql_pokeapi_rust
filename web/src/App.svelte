<script lang="ts">
  import Pokedex from "./Pokedex.svelte";

  import { initClient } from "@urql/svelte";
  import { operationStore, query } from "@urql/svelte";

  initClient({
    url: "https://graphql-pokeapi.herokuapp.com/graphql",
    // url: "http://localhost/graphql",
  });

  const qres = operationStore(
    `
    {
      apiVersion
    }
    `
  );
  query(qres);
</script>

<main>
  {#if $qres.fetching}
    <p>Loading...</p>
  {:else if $qres.error}
    <p>Oh no... {$qres.error.message}</p>
  {:else}
    <div class="container">
      <div class="fullWrap">
        <Pokedex />
      </div>

      <footer>
        <a
          aria-label="Credits"
          class="version"
          href="https://codepen.io/Bidji/pen/MYdPwo">Credits</a
        >
        <a
          aria-label="Source Code on Github"
          class="version"
          href="https://github.com/reime005/graphql_pokeapi_rust"
          >Code on Github (v{qres.data.apiVersion})</a
        >
      </footer>
    </div>
  {/if}
</main>

<style>
  main {
    overflow-y: scroll;
    width: 100%;
    min-height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
  }

  footer {
    margin-top: 2em;
    display: flex;
    justify-content: space-between;
    padding: 1vw;
    width: 100%;
  }

  .container {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    width: 100%;
    height: 100%;
    padding: 2vw;
  }

  .fullWrap {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .version {
    font-size: 0.8rem;
    color: var(--grey);
    align-self: flex-end;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
  }
</style>
