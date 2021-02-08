# GraphQL PokeAPI Wrapper

This project is still experimental / WIP

Unofficial GraphQL API for the [PokeAPI](https://pokeapi.co).

Endpoint is https://graphql-pokeapi.herokuapp.com/graphql.

Svelte PokeDex UI is [https://graphql-pokeapi-rust.reime005.vercel.app](https://graphql-pokeapi-rust.reime005.vercel.app).

GraphiQL Endpoint is https://graphql-pokeapi.herokuapp.com/graphiql (pointing to a local instance of https://pokeapi.co/api/v2/).

## Local Setup

For local development, I recommend using your local REST endpoint. For that, do the following:

    git submodule update --init --recursive
    docker-compose -f ./pokeapi/docker-compose.yml up -d
    docker-compose -f ./pokeapi/docker-compose.yml exec -T app python manage.py migrate --settings=config.docker-compose
    docker-compose -f ./pokeapi/docker-compose.yml exec -T app sh -c 'echo "from data.v2.build import build_all; build_all()" | python manage.py shell --settings=config.docker-compose'

The `.env` file can be used to point against a different REST API. For that, set the `POKERUST_ENDPOINT` env variable accordingly. For local development using `docker-compose`, that address would be `http://localhost:80/api/v2/`.

Then, use `cargo run` and navigate to http://localhost:8080/graphiql (or a different port that you specified via `PORT`) and test your GraphQL queries. If something fails, it might be that you have to run the rust program with the release flag `cargo run --release`.

## Query Example

The following query:

    {
      pokemonByName(name: "pikachu") {
        height
        sprites {
          other {
            officialArtwork {
              frontDefault
            }
          }
        }
      }
    }

Would yield the following result:

    {
      "data": {
        "pokemonByName": {
          "height": "4",
          "sprites": {
            "other": {
              "officialArtwork": {
                "frontDefault": "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/other/official-artwork/25.png"
              }
            }
          }
        }
      }
    }

<img width="350px" height="350px" alt="Pokemon Pikachu" src="https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/other/official-artwork/25.png" />

The positive aspects, compared to a REST API, are:

- only receiving the specified, necessary fields
- possibility of querying multiple different things (like pokemon and berries) in one request, so that the end user just needs to to one request instead of two
- self-describing API that makes it easier to explore
