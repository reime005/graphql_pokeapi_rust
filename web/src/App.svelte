<script lang="ts">
  import Pokedex from "./Pokedex.svelte";

  import { initClient } from "@urql/svelte";
  import { operationStore, query } from "@urql/svelte";
  import Data from "./Data.svelte";

  initClient({
    // url: "https://graphql-pokeapi.herokuapp.com/graphql",
    url: "http://localhost:8080/graphql",
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
        <Pokedex name="pikachu" />
      </div>

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
    </div>
  {/if}
</main>

<style>
  main {
    height: 100vh;
    max-height: 100vh;
    overflow: hidden;
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

  @media (min-width: 640px) {
    main {
      background-color: var(--red);
      max-width: var(--mobileBreak);
    }
  }
</style>
