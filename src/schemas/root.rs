use juniper::{
    graphql_object, EmptyMutation, EmptySubscription, FieldResult, GraphQLEnum, GraphQLInputObject,
    GraphQLObject, ScalarValue,
};

use super::berry::{GraphedBerry, GraphedBerryFlavor};
use super::generic::*;
use super::pokemon::*;
use super::resource_list::*;
use pokerust::{
    Berry, BerryFirmness, BerryFlavor, BerryFlavorMap, Characteristic, EggGroup, Endpoint, FromId,
    FromName, Item, List, NamedAPIResourceList, Pokemon, PokemonAbility, PokemonForm,
    PokemonSpecies,
};

use std::convert::TryInto;

pub struct Context {}

impl juniper::Context for Context {}

pub struct QueryRoot;

const DEFAULT_OFFSET: i32 = 0;
const DEFAULT_LIMIT: i32 = 20;
const VERSION: &str = "0.1";

#[graphql_object(
    // Here we specify the context type for the object.
    // We need to do this in every type that
    // needs access to the context.
    context = Context,
)]
impl QueryRoot {
    fn apiVersion() -> &str {
        VERSION
    }

    fn defaultOffset() -> i32 {
        DEFAULT_OFFSET
    }

    fn defaultLimit() -> i32 {
        DEFAULT_LIMIT
    }

    #[graphql(description = "Retrieve a list of resources")]
    fn berries(
        _context: &Context,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> FieldResult<GraphedNamedAPIResourceList> {
        let objects = Berry::list(
            offset.unwrap_or(DEFAULT_OFFSET).try_into()?,
            limit.unwrap_or(DEFAULT_LIMIT).try_into()?,
        )?;
        Ok(GraphedNamedAPIResourceList::from(objects))
    }

    #[graphql(description = "Retrieve a list of resources")]
    fn berry_firmnesses(
        _context: &Context,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> FieldResult<GraphedNamedAPIResourceList> {
        let objects = BerryFirmness::list(
            offset.unwrap_or(DEFAULT_OFFSET).try_into()?,
            limit.unwrap_or(DEFAULT_LIMIT).try_into()?,
        )?;
        Ok(GraphedNamedAPIResourceList::from(objects))
    }

    #[graphql(description = "Retrieve a list of resources")]
    fn berry_flavors(
        _context: &Context,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> FieldResult<GraphedNamedAPIResourceList> {
        let objects = BerryFlavor::list(
            offset.unwrap_or(DEFAULT_OFFSET).try_into()?,
            limit.unwrap_or(DEFAULT_LIMIT).try_into()?,
        )?;
        Ok(GraphedNamedAPIResourceList::from(objects))
    }

    #[graphql(description = "Retrieve a list of pokemons")]
    fn pokemons(
        _context: &Context,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> FieldResult<GraphedNamedAPIResourceList> {
        let objects = Pokemon::list(
            offset.unwrap_or(DEFAULT_OFFSET).try_into()?,
            limit.unwrap_or(DEFAULT_LIMIT).try_into()?,
        )?;
        Ok(GraphedNamedAPIResourceList::from(objects))
    }

    #[graphql(description = "Retrieve a list of pokemon forms")]
    fn pokemon_forms(
        _context: &Context,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> FieldResult<GraphedNamedAPIResourceList> {
        let objects = PokemonForm::list(
            offset.unwrap_or(DEFAULT_OFFSET).try_into()?,
            limit.unwrap_or(DEFAULT_LIMIT).try_into()?,
        )?;
        Ok(GraphedNamedAPIResourceList::from(objects))
    }

    #[graphql(description = "Retrieve a list of pokemon characteristics")]
    fn pokemon_characteristics(
        _context: &Context,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> FieldResult<GraphedAPIResourceList> {
        let objects = Characteristic::list(
            offset.unwrap_or(DEFAULT_OFFSET).try_into()?,
            limit.unwrap_or(DEFAULT_LIMIT).try_into()?,
        )?;
        Ok(GraphedAPIResourceList::from(objects))
    }

    #[graphql(description = "Retrieve a list of pokemon egg groups")]
    fn pokemon_egggroups(
        _context: &Context,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> FieldResult<GraphedNamedAPIResourceList> {
        let objects = EggGroup::list(
            offset.unwrap_or(DEFAULT_OFFSET).try_into()?,
            limit.unwrap_or(DEFAULT_LIMIT).try_into()?,
        )?;
        Ok(GraphedNamedAPIResourceList::from(objects))
    }

    #[graphql(description = "Retrieve a berry information by its name")]
    fn berry_by_name(_context: &Context, name: String) -> FieldResult<GraphedBerry> {
        let berry = Berry::from_name(&name)?;
        Ok(GraphedBerry::from(berry))
    }

    #[graphql(description = "Retrieve a berry information by its id")]
    fn berry_by_id(_context: &Context, id: String) -> FieldResult<GraphedBerry> {
        let berry = Berry::from_id(id.parse()?)?;
        Ok(GraphedBerry::from(berry))
    }

    #[graphql(description = "Retrieve a pokemon information by its name")]
    fn pokemon_by_name(_context: &Context, name: String) -> FieldResult<GraphedPokemon> {
        let obj = Pokemon::from_name(&name)?;
        Ok(GraphedPokemon::from(obj))
    }

    #[graphql(description = "Retrieve a pokemon information by its name")]
    fn pokemon_by_id(_context: &Context, id: String) -> FieldResult<GraphedPokemon> {
        let obj = Pokemon::from_id(id.parse()?)?;
        Ok(GraphedPokemon::from(obj))
    }

    #[graphql(description = "Retrieve a pokemon form by its name")]
    fn pokemon_form_by_name(_context: &Context, name: String) -> FieldResult<GraphedPokemonForm> {
        let obj = PokemonForm::from_name(&name)?;
        Ok(GraphedPokemonForm::from(obj))
    }

    #[graphql(description = "Retrieve a pokemon form by its name")]
    fn pokemon_form_by_id(_context: &Context, id: String) -> FieldResult<GraphedPokemonForm> {
        let obj = PokemonForm::from_id(id.parse()?)?;
        Ok(GraphedPokemonForm::from(obj))
    }

    #[graphql(description = "Retrieve a pokemon species by its name")]
    fn pokemon_species_by_name(
        _context: &Context,
        name: String,
    ) -> FieldResult<GraphedPokemonSpecies> {
        let obj = PokemonSpecies::from_name(&name)?;
        Ok(GraphedPokemonSpecies::from(obj))
    }

    #[graphql(description = "Retrieve a pokemon species by its id")]
    fn pokemon_species_by_id(_context: &Context, id: String) -> FieldResult<GraphedPokemonSpecies> {
        let obj = PokemonSpecies::from_id(id.parse()?)?;
        Ok(GraphedPokemonSpecies::from(obj))
    }

    #[graphql(description = "Retrieve a pokemon characteristic by its id")]
    fn pokemon_characteristic_by_id(
        _context: &Context,
        id: String,
    ) -> FieldResult<GraphedCharacteristic> {
        let obj = Characteristic::from_id(id.parse()?)?;
        Ok(GraphedCharacteristic::from(obj))
    }

    #[graphql(description = "Retrieve a pokemon egg group by its name")]
    fn pokemon_eggroup_by_name(context: &Context, name: String) -> FieldResult<GraphedEggGroup> {
        let obj = EggGroup::from_name(&name)?;
        Ok(GraphedEggGroup::from(obj))
    }

    #[graphql(description = "Retrieve a pokemon egg group by its id")]
    fn pokemon_egggroup_by_id(_context: &Context, id: String) -> FieldResult<GraphedEggGroup> {
        let obj = EggGroup::from_id(id.parse()?)?;
        Ok(GraphedEggGroup::from(obj))
    }
}

// pub struct MutationRoot;

// #[graphql_object(
//     context = Cowntext,
// )]
// impl MutationRoot {
//     fn apiVersion() -> &str {
//         VERSION
//     }
// }

pub type Schema =
    juniper::RootNode<'static, QueryRoot, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, EmptyMutation::new(), EmptySubscription::new())
}
