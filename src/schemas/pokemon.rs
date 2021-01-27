use juniper::{
  graphql_interface, graphql_object, EmptySubscription, FieldResult, GraphQLEnum,
  GraphQLInputObject, GraphQLObject, ScalarValue,
};

use crate::schemas::berry::GraphedBerryFlavor;
use crate::schemas::contests::GraphedContestType;
use crate::schemas::encounters::*;
use crate::schemas::games::*;
use crate::schemas::generic::*;
use crate::schemas::root::Context;

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedAbility {
  pub id: String,
  pub name: String,
  pub is_main_series: bool,
  // pub generation: GraphedGeneration,
  pub names: Vec<GraphedName>,
  pub effect_entries: Vec<GraphedVerboseEffect>,
  pub effect_changes: Vec<GraphedAbilityEffectChange>,
  pub flavor_text_entries: Vec<GraphedAbilityFlavorText>,
  pub pokemon: Vec<GraphedAbilityPokemon>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedAbilityEffectChange {
  pub effect_entries: Vec<GraphedEffect>,
  // pub version_group: GraphedVersionGroup,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedAbilityFlavorText {
  pub flavor_text: String,
  pub language: GraphedLanguage,
  // pub version_group: GraphedVersionGroup,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedAbilityPokemon {
  pub is_hidden: bool,
  pub slot: String,
  pub pokemon: GraphedPokemon,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedCharacteristic {
  pub id: String,
  pub gene_modulo: String,
  pub possible_values: Vec<String>,
  pub highest_stat: GraphedStat,        // not documented
  pub descriptions: GraphedDescription, // not documented
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedEggGroup {
  pub id: String,
  pub name: String,
  pub names: Vec<GraphedName>,
  pub pokemon_species: Vec<GraphedPokemonSpecies>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedGender {
  pub id: String,
  pub name: String,
  pub pokemon_species_details: Vec<GraphedPokemonSpeciesGender>,
  pub required_for_evolution: Vec<GraphedPokemonSpecies>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedPokemonSpeciesGender {
  pub rate: String,
  pub pokemon_species: GraphedPokemonSpecies,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedGrowthRate {
  pub id: String,
  pub name: String,
  pub formula: String,
  pub descriptions: Vec<GraphedDescription>,
  pub levels: Vec<GraphedGrowthRateExperienceLevel>,
  pub pokemon_species: Vec<GraphedPokemonSpecies>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedGrowthRateExperienceLevel {
  pub level: String,
  pub experience: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedNature {
  pub id: String,
  pub name: String,
  pub decreased_stat: Option<GraphedStat>,
  pub increased_stat: Option<GraphedStat>,
  pub hates_flavor: Option<GraphedBerryFlavor>,
  pub likes_flavor: Option<GraphedBerryFlavor>,
  pub pokeathlon_stat_changes: Vec<GraphedNatureStatChange>,
  pub move_battle_style_preferences: Vec<GraphedMoveBattleStylePreference>,
  pub names: Vec<GraphedName>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedNatureStatChange {
  pub max_change: String,
  pub pokeathlon_stat: GraphedPokeathlonStat,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedMoveBattleStylePreference {
  pub low_hp_preference: String,
  pub high_hp_preference: String,
  // pub move_battle_style: GrapedMoveBattleStyle,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedPokeathlonStat {
  pub id: String,
  pub name: String,
  pub names: Vec<GraphedName>,
  pub affecting_natures: GraphedNaturePokeathlonStatAffectSets,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedNaturePokeathlonStatAffectSets {
  pub increase: Vec<GraphedNaturePokeathlonStatAffect>,
  pub decrease: Vec<GraphedNaturePokeathlonStatAffect>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedNaturePokeathlonStatAffect {
  pub max_change: String,
  pub nature: GraphedNature,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedPokemon {
  pub id: String,
  pub name: String,
  pub base_experience: String,
  pub height: String,
  pub is_default: bool,
  pub order: String,
  pub weight: String,
  pub abilities: Vec<GraphedPokemonAbility>,
  pub forms: Vec<GraphedPokemonForm>,
  pub game_indices: Vec<GraphedVersionGameIndex>,
  pub held_items: Vec<GraphedPokemonHeldItem>,
  pub location_area_encounters: String, // TODO: implement a way to retrieve these
  pub moves: Vec<GraphedPokemonMove>,
  pub sprites: GraphedPokemonSprites,
  pub species: GraphedPokemonSpecies,
  pub stats: Vec<GraphedPokemonStat>,
  pub types: Vec<GraphedPokemonType>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedPokemonAbility {
  pub is_hidden: bool,
  pub slot: String,
  pub ability: GraphedAbility,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedPokemonType {
  pub slot: String,
  // TODO: rename
  pub type_: GraphedType,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedPokemonHeldItem {
  // pub item: GraphedItem,
  pub version_details: Vec<GraphedPokemonHeldItemVersion>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedPokemonHeldItemVersion {
  pub version: GraphedVersion,
  pub rarity: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedPokemonMove {
  // TODO: rename
  // pub move_: GraphedMove,
  pub version_group_details: Vec<GraphedPokemonMoveVersion>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedPokemonMoveVersion {
  // pub move_learn_method: GraphedMoveLearnMethod,
  pub version_group: GraphedVersionGroup,
  pub level_learned_at: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedPokemonStat {
  pub stat: GraphedStat,
  pub effort: String,
  pub base_stat: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedPokemonSprites {
  pub front_default: Option<String>,
  pub front_shiny: Option<String>,
  pub front_female: Option<String>,
  pub front_shiny_female: Option<String>,
  pub back_default: Option<String>,
  pub back_shiny: Option<String>,
  pub back_female: Option<String>,
  pub back_shiny_female: Option<String>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedLocationAreaEncounter {
  // pub location_area: GraphedLocationArea,
  pub version_details: GraphedVersionEncounterDetail,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedPokemonColor {
  pub id: String,
  pub name: String,
  pub names: Vec<GraphedName>,
  pub pokemon_species: Vec<GraphedPokemonSpecies>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedPokemonForm {
  pub id: String,
  pub name: String,
  pub order: String,
  pub form_order: String,
  pub is_default: bool,
  pub is_battle_only: bool,
  pub is_mega: bool,
  pub form_name: String,
  pub pokemon: GraphedPokemon,
  pub sprites: GraphedPokemonFormSprites,
  // pub version_group: GraphedVersionGroup,
  pub names: Vec<GraphedName>,
  pub form_names: Vec<GraphedName>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedPokemonFormSprites {
  pub front_default: Option<String>,
  pub front_shiny: Option<String>,
  pub back_default: Option<String>,
  pub back_shiny: Option<String>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedPokemonHabitat {
  pub id: String,
  pub name: String,
  pub names: Vec<GraphedName>,
  pub pokemon_species: Vec<GraphedPokemonSpecies>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedPokemonShape {
  pub id: String,
  pub name: String,
  pub awesome_names: Vec<GraphedAwesomeName>,
  pub names: Vec<GraphedName>,
  pub pokemon_species: Vec<GraphedPokemonSpecies>, // incorrectly documented as list PokemonSpecies
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedAwesomeName {
  pub awesome_name: String,
  pub language: GraphedLanguage,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedPokemonSpecies {
  pub id: String,
  pub name: String,
  pub order: String,
  pub gender_rate: String,
  pub capture_rate: String,
  pub base_happiness: String,
  pub is_baby: bool,
  pub hatch_counter: String,
  pub has_gender_differences: bool,
  pub forms_switchable: bool,
  pub growth_rate: GraphedGrowthRate,
  pub pokedex_numbers: GraphedPokemonSpeciesDexEntry,
  pub egg_groups: GraphedEggGroup,
  pub color: GraphedPokemonColor,
  pub shape: GraphedPokemonShape,
  // pub evolves_from_species: Option<GraphedPokemonSpecies>,
  // pub evolution_chain: GraphedEvolutionChain,
  pub habitat: Option<GraphedPokemonHabitat>,
  // pub generation: GraphedGeneration,
  pub names: Vec<GraphedName>,
  pub pal_park_encounters: Vec<GraphedPalParkEncounterArea>,
  pub flavor_text_entries: Vec<GraphedFlavorText>,
  pub form_descriptions: Vec<GraphedDescription>,
  pub genera: Vec<GraphedGenus>,
  pub varieties: Vec<GraphedPokemonSpeciesVariety>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedGenus {
  pub genus: String,
  pub language: GraphedLanguage,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedPokemonSpeciesDexEntry {
  pub entry_number: String,
  // pub pokedex: GraphedPokedex,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedPalParkEncounterArea {
  pub base_score: String,
  pub rate: String,
  // pub area: GraphedPalParkArea,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedPokemonSpeciesVariety {
  pub is_default: bool,
  pub pokemon: GraphedPokemon,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedStat {
  pub id: String,
  pub name: String,
  pub game_index: String,
  pub is_battle_only: bool,
  pub affecting_moves: GraphedMoveStatAffectSets,
  pub affecting_natures: GraphedNatureStatAffectSets,
  pub characteristics: Vec<GraphedCharacteristic>, // incorrectly documented as APIResource
  // pub move_damage_class: Option<GraphedMoveDamageClass>,
  pub names: Vec<GraphedName>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedMoveStatAffectSets {
  pub increase: Vec<GraphedMoveStatAffect>,
  pub decrease: Vec<GraphedMoveStatAffect>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedMoveStatAffect {
  pub change: String,
  // TODO: rename
  // pub move_: GraphedMove,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedNatureStatAffectSets {
  pub increase: Vec<GraphedMoveStatAffect>,
  pub decrease: Vec<GraphedMoveStatAffect>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedType {
  pub id: String,
  pub name: String,
  // pub damage_relations: TypeRelations,
  // pub game_indices: Vec<GenerationGameIndex>,
  // pub generation: NamedAPIResource<Generation>,
  // pub move_damage_class: Option<NamedAPIResource<MoveDamageClass>>,
  // pub names: Vec<Name>,
  // pub pokemon: Vec<TypePokemon>,
  // pub moves: Vec<NamedAPIResource<Move>>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedTypePokemon {
  pub slot: String,
  pub pokemon: GraphedPokemon,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedTypeRelations {
  pub no_damage_to: Vec<GraphedType>,
  pub half_damage_to: Vec<GraphedType>,
  pub double_damage_to: Vec<GraphedType>,
  pub no_damage_from: Vec<GraphedType>,
  pub half_damage_from: Vec<GraphedType>,
  pub double_damage_from: Vec<GraphedType>,
}
