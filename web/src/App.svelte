<script lang="ts">
  import Data from "./Data.svelte";
  // import Pokedex from "./Pokedex.svelte";

  import { initClient } from "@urql/svelte";

  initClient({
    url: "https://graphql-pokeapi.herokuapp.com/graphql",
  });
</script>

<main>
  <Data />
  <!-- <Pokedex /> -->
</main>

<style>

  main {
    height: 100vh;
    max-height: 100vh;
    overflow: hidden;
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
