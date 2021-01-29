use std::borrow::Borrow;

use juniper::{
    graphql_interface, graphql_object, EmptySubscription, FieldResult, GraphQLEnum,
    GraphQLInputObject, GraphQLObject, ScalarValue,
};

use pokerust::*;

use crate::schemas::berry::GraphedBerryFlavor;
use crate::schemas::contests::GraphedContestType;
use crate::schemas::encounters::*;
use crate::schemas::evolution::*;
use crate::schemas::games::*;
use crate::schemas::generic::*;
use crate::schemas::root::Context;

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedAbility {
    pub id: String,
    pub name: String,
    pub is_main_series: bool,
    pub generation: GraphedNamedAPIResource,
    pub names: Vec<GraphedName>,
    pub effect_entries: Vec<GraphedVerboseEffect>,
    pub effect_changes: Vec<GraphedAbilityEffectChange>,
    pub flavor_text_entries: Vec<GraphedAbilityFlavorText>,
    pub pokemon: Vec<GraphedAbilityPokemon>,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedAbilityEffectChange {
    pub effect_entries: Vec<GraphedEffect>,
    pub version_group: GraphedNamedAPIResource,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedAbilityFlavorText {
    pub flavor_text: String,
    pub language: GraphedNamedAPIResource,
    pub version_group: GraphedNamedAPIResource,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedAbilityPokemon {
    pub is_hidden: bool,
    pub slot: String,
    pub pokemon: GraphedNamedAPIResource,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedCharacteristic {
    pub id: String,
    pub gene_modulo: String,
    pub possible_values: Vec<String>,
    pub highest_stat: GraphedNamedAPIResource, // not documented
    pub descriptions: Vec<GraphedDescription>, // not documented
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedEggGroup {
    pub id: String,
    pub name: String,
    pub names: Vec<GraphedName>,
    pub pokemon_species: Vec<GraphedNamedAPIResource>,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedGender {
    pub id: String,
    pub name: String,
    pub pokemon_species_details: Vec<GraphedPokemonSpeciesGender>,
    pub required_for_evolution: Vec<GraphedNamedAPIResource>,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedPokemonSpeciesGender {
    pub rate: String,
    pub pokemon_species: GraphedNamedAPIResource,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedGrowthRate {
    pub id: String,
    pub name: String,
    pub formula: String,
    pub descriptions: Vec<GraphedDescription>,
    pub levels: Vec<GraphedGrowthRateExperienceLevel>,
    pub pokemon_species: Vec<GraphedNamedAPIResource>,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedGrowthRateExperienceLevel {
    pub level: String,
    pub experience: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedNature {
    pub id: String,
    pub name: String,
    pub decreased_stat: Option<GraphedNamedAPIResource>,
    pub increased_stat: Option<GraphedNamedAPIResource>,
    pub hates_flavor: Option<GraphedNamedAPIResource>,
    pub likes_flavor: Option<GraphedNamedAPIResource>,
    pub pokeathlon_stat_changes: Vec<GraphedNatureStatChange>,
    pub move_battle_style_preferences: Vec<GraphedMoveBattleStylePreference>,
    pub names: Vec<GraphedName>,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedNatureStatChange {
    pub max_change: String,
    pub pokeathlon_stat: GraphedNamedAPIResource,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedMoveBattleStylePreference {
    pub low_hp_preference: String,
    pub high_hp_preference: String,
    pub move_battle_style: GraphedNamedAPIResource,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedPokeathlonStat {
    pub id: String,
    pub name: String,
    pub names: Vec<GraphedName>,
    pub affecting_natures: GraphedNaturePokeathlonStatAffectSets,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedNaturePokeathlonStatAffectSets {
    pub increase: Vec<GraphedNaturePokeathlonStatAffect>,
    pub decrease: Vec<GraphedNaturePokeathlonStatAffect>,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedNaturePokeathlonStatAffect {
    pub max_change: String,
    pub nature: GraphedNamedAPIResource,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedPokemon {
    pub id: String,
    pub name: String,
    pub base_experience: String,
    pub height: String,
    pub is_default: bool,
    pub order: String,
    pub weight: String,
    pub abilities: Vec<GraphedPokemonAbility>,
    pub forms: Vec<GraphedNamedAPIResource>,
    pub game_indices: Vec<GraphedVersionGameIndex>,
    pub held_items: Vec<GraphedPokemonHeldItem>,
    pub location_area_encounters: String, // TODO: implement a way to retrieve these
    pub moves: Vec<GraphedPokemonMove>,
    pub sprites: GraphedPokemonSprites,
    pub species: GraphedNamedAPIResource,
    pub stats: Vec<GraphedPokemonStat>,
    pub types: Vec<GraphedPokemonType>,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedPokemonAbility {
    pub is_hidden: bool,
    pub slot: String,
    pub ability: GraphedNamedAPIResource,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedPokemonType {
    pub slot: String,
    #[graphql(name = "type")]
    pub type_: GraphedNamedAPIResource,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedPokemonHeldItem {
    pub item: GraphedNamedAPIResource,
    pub version_details: Vec<GraphedPokemonHeldItemVersion>,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedPokemonHeldItemVersion {
    pub version: GraphedNamedAPIResource,
    pub rarity: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedPokemonMove {
    #[graphql(name = "move")]
    pub move_: GraphedNamedAPIResource,
    pub version_group_details: Vec<GraphedPokemonMoveVersion>,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedPokemonMoveVersion {
    pub move_learn_method: GraphedNamedAPIResource,
    pub version_group: GraphedNamedAPIResource,
    pub level_learned_at: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedPokemonStat {
    pub stat: GraphedNamedAPIResource,
    pub effort: String,
    pub base_stat: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedPokemonSprites {
    pub front_default: Option<String>,
    pub front_shiny: Option<String>,
    pub front_female: Option<String>,
    pub front_shiny_female: Option<String>,
    pub back_default: Option<String>,
    pub back_shiny: Option<String>,
    pub back_female: Option<String>,
    pub back_shiny_female: Option<String>,
    pub other: GraphedOtherSprites,
    pub versions: GraphedVersionSprites,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedVersionSidesSprites {
    pub front_default: Option<String>,
    pub front_gray: Option<String>,
    pub front_shiny: Option<String>,
    pub front_female: Option<String>,
    pub front_shiny_female: Option<String>,
    pub back_default: Option<String>,
    pub back_gray: Option<String>,
    pub back_shiny: Option<String>,
    pub back_female: Option<String>,
    pub back_shiny_female: Option<String>,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedOtherSprites {
    pub dream_world: GraphedDreamWorldSprites,
    pub official_artwork: GraphedOfficialArtWorkSprites,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedDreamWorldSprites {
    pub front_default: Option<String>,
    pub front_female: Option<String>,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedOfficialArtWorkSprites {
    pub front_default: Option<String>,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedVersionSprites {
    pub generation_i: GraphedGeneration1Sprites,
    pub generation_ii: GraphedGeneration2Sprites,
    pub generation_iii: GraphedGeneration3Sprites,
    pub generation_iv: GraphedGeneration4Sprites,
    pub generation_v: GraphedGeneration5Sprites,
    pub generation_vi: GraphedGeneration6Sprites,
    pub generation_vii: GraphedGeneration7Sprites,
    pub generation_viii: GraphedGeneration8Sprites,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedGeneration1Sprites {
    pub red_blue: GraphedVersionSidesSprites,
    pub yellow: GraphedVersionSidesSprites,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedGeneration2Sprites {
    pub crystal: GraphedVersionSidesSprites,
    pub gold: GraphedVersionSidesSprites,
    pub silver: GraphedVersionSidesSprites,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedGeneration3Sprites {
    pub emerald: GraphedVersionSidesSprites,
    pub firered_leafgreen: GraphedVersionSidesSprites,
    pub ruby_sapphire: GraphedVersionSidesSprites,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedGeneration4Sprites {
    pub diamond_pearl: GraphedVersionSidesSprites,
    pub heartgold_soulsilver: GraphedVersionSidesSprites,
    pub platinum: GraphedVersionSidesSprites,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedGeneration5Sprites {
    pub black_white: GraphedWithAnimatedVersionSidesSprites,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedWithAnimatedVersionSidesSprites {
    pub front_default: Option<String>,
    pub front_gray: Option<String>,
    pub front_shiny: Option<String>,
    pub front_female: Option<String>,
    pub front_shiny_female: Option<String>,
    pub back_default: Option<String>,
    pub back_gray: Option<String>,
    pub back_shiny: Option<String>,
    pub back_female: Option<String>,
    pub back_shiny_female: Option<String>,
    pub animated: GraphedVersionSidesSprites,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedGeneration6Sprites {
    pub omegaruby_alphasapphire: GraphedVersionSidesSprites,
    pub x_y: GraphedVersionSidesSprites,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedGeneration7Sprites {
    pub ultra_sun_ultra_moon: GraphedVersionSidesSprites,
    pub icons: GraphedVersionSidesSprites,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedGeneration8Sprites {
    pub icons: GraphedVersionSidesSprites,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedLocationAreaEncounter {
    pub location_area: GraphedNamedAPIResource,
    pub version_details: Vec<GraphedVersionEncounterDetail>,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedPokemonColor {
    pub id: String,
    pub name: String,
    pub names: Vec<GraphedName>,
    pub pokemon_species: Vec<GraphedNamedAPIResource>,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedPokemonForm {
    pub id: String,
    pub name: String,
    pub order: String,
    pub form_order: String,
    pub is_default: bool,
    pub is_battle_only: bool,
    pub is_mega: bool,
    pub form_name: String,
    pub pokemon: GraphedNamedAPIResource,
    pub sprites: GraphedPokemonFormSprites,
    pub version_group: GraphedNamedAPIResource,
    pub names: Vec<GraphedName>,
    pub form_names: Vec<GraphedName>,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedPokemonFormSprites {
    pub front_default: Option<String>,
    pub front_shiny: Option<String>,
    pub back_default: Option<String>,
    pub back_shiny: Option<String>,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedPokemonHabitat {
    pub id: String,
    pub name: String,
    pub names: Vec<GraphedName>,
    pub pokemon_species: Vec<GraphedNamedAPIResource>,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedPokemonShape {
    pub id: String,
    pub name: String,
    pub awesome_names: Vec<GraphedAwesomeName>,
    pub names: Vec<GraphedName>,
    pub pokemon_species: Vec<GraphedNamedAPIResource>, // incorrectly documented as list PokemonSpecies
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedAwesomeName {
    pub awesome_name: String,
    pub language: GraphedNamedAPIResource,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
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
    pub growth_rate: GraphedNamedAPIResource,
    pub pokedex_numbers: Vec<GraphedPokemonSpeciesDexEntry>,
    pub egg_groups: Vec<GraphedNamedAPIResource>,
    pub color: GraphedNamedAPIResource,
    pub shape: GraphedNamedAPIResource,
    pub evolves_from_species: Option<GraphedNamedAPIResource>,
    pub evolution_chain: GraphedAPIResource,
    pub habitat: Option<GraphedNamedAPIResource>,
    pub generation: GraphedNamedAPIResource,
    pub names: Vec<GraphedName>,
    pub pal_park_encounters: Vec<GraphedPalParkEncounterArea>,
    pub flavor_text_entries: Vec<GraphedFlavorText>,
    pub form_descriptions: Vec<GraphedDescription>,
    pub genera: Vec<GraphedGenus>,
    pub varieties: Vec<GraphedPokemonSpeciesVariety>,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedGenus {
    pub genus: String,
    pub language: GraphedNamedAPIResource,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedPokemonSpeciesDexEntry {
    pub entry_number: String,
    pub pokedex: GraphedNamedAPIResource,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedPalParkEncounterArea {
    pub base_score: String,
    pub rate: String,
    pub area: GraphedNamedAPIResource,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedPokemonSpeciesVariety {
    pub is_default: bool,
    pub pokemon: GraphedNamedAPIResource,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedStat {
    pub id: String,
    pub name: String,
    pub game_index: String,
    pub is_battle_only: bool,
    pub affecting_moves: GraphedMoveStatAffectSets,
    pub affecting_natures: GraphedNatureStatAffectSets,
    pub characteristics: Vec<GraphedNamedAPIResource>, // incorrectly documented as APIResource
    pub move_damage_class: Option<GraphedNamedAPIResource>,
    pub names: Vec<GraphedName>,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedMoveStatAffectSets {
    pub increase: Vec<GraphedMoveStatAffect>,
    pub decrease: Vec<GraphedMoveStatAffect>,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedMoveStatAffect {
    pub change: String,
    // #[graphql(name = "move")]
    pub move_: GraphedNamedAPIResource,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedNatureStatAffectSets {
    pub increase: Vec<GraphedNamedAPIResource>,
    pub decrease: Vec<GraphedNamedAPIResource>,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedType {
    pub id: String,
    pub name: String,
    pub damage_relations: GraphedTypeRelations,
    pub game_indices: Vec<GraphedGenerationGameIndex>,
    pub generation: GraphedNamedAPIResource,
    pub move_damage_class: Option<GraphedNamedAPIResource>,
    pub names: Vec<GraphedName>,
    pub pokemon: Vec<GraphedTypePokemon>,
    pub moves: Vec<GraphedNamedAPIResource>,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedTypePokemon {
    pub slot: String,
    pub pokemon: GraphedNamedAPIResource,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedTypeRelations {
    pub no_damage_to: Vec<GraphedNamedAPIResource>,
    pub half_damage_to: Vec<GraphedNamedAPIResource>,
    pub double_damage_to: Vec<GraphedNamedAPIResource>,
    pub no_damage_from: Vec<GraphedNamedAPIResource>,
    pub half_damage_from: Vec<GraphedNamedAPIResource>,
    pub double_damage_from: Vec<GraphedNamedAPIResource>,
}

impl From<Pokemon> for GraphedPokemon {
    fn from(obj: Pokemon) -> Self {
        GraphedPokemon {
            id: obj.id.to_string(),
            name: obj.name,
            base_experience: obj.base_experience.to_string(),
            height: obj.height.to_string(),
            is_default: obj.is_default,
            order: obj.order.to_string(),
            weight: obj.weight.to_string(),
            location_area_encounters: obj.location_area_encounters,
            abilities: conv_vector(&obj.abilities),
            forms: conv_vector(&obj.forms),
            game_indices: conv_vector(&obj.game_indices),
            held_items: conv_vector(&obj.held_items),
            moves: conv_vector(&obj.moves),
            sprites: GraphedPokemonSprites::from(obj.sprites),
            species: GraphedNamedAPIResource::from(&obj.species),
            stats: conv_vector(&obj.stats),
            types: conv_vector(&obj.types),
        }
    }
}

impl From<PokemonSpecies> for GraphedPokemonSpecies {
    fn from(obj: PokemonSpecies) -> Self {
        GraphedPokemonSpecies {
            id: obj.id.to_string(),
            name: obj.name,
            order: obj.order.to_string(),
            gender_rate: obj.gender_rate.to_string(),
            capture_rate: obj.capture_rate.to_string(),
            base_happiness: obj.base_happiness.to_string(),
            is_baby: obj.is_baby,
            hatch_counter: obj.hatch_counter.to_string(),
            has_gender_differences: obj.has_gender_differences,
            forms_switchable: obj.forms_switchable,
            growth_rate: GraphedNamedAPIResource::from(&obj.growth_rate),
            pokedex_numbers: conv_vector(&obj.pokedex_numbers),
            egg_groups: conv_vector(&obj.egg_groups),
            color: GraphedNamedAPIResource::from(&obj.color),
            shape: GraphedNamedAPIResource::from(&obj.shape),
            evolves_from_species: match obj.evolves_from_species {
                Some(val) => Some(GraphedNamedAPIResource::from(&val)),
                None => None,
            },
            evolution_chain: GraphedAPIResource::from(&obj.evolution_chain),
            habitat: match obj.habitat {
                Some(val) => Some(GraphedNamedAPIResource::from(&val)),
                None => None,
            },
            generation: GraphedNamedAPIResource::from(&obj.generation),
            names: conv_vector(&obj.names),
            pal_park_encounters: conv_vector(&obj.pal_park_encounters),
            flavor_text_entries: conv_vector(&obj.flavor_text_entries),
            form_descriptions: conv_vector(&obj.form_descriptions),
            genera: conv_vector(&obj.genera),
            varieties: conv_vector(&obj.varieties),
        }
    }
}

impl From<PokemonHeldItemVersion> for GraphedPokemonHeldItemVersion {
    fn from(obj: PokemonHeldItemVersion) -> Self {
        GraphedPokemonHeldItemVersion {
            rarity: obj.rarity.to_string(),
            version: GraphedNamedAPIResource::from(&obj.version),
        }
    }
}

impl From<PokemonMoveVersion> for GraphedPokemonMoveVersion {
    fn from(obj: PokemonMoveVersion) -> Self {
        GraphedPokemonMoveVersion {
            level_learned_at: obj.level_learned_at.to_string(),
            move_learn_method: GraphedNamedAPIResource::from(&obj.move_learn_method),
            version_group: GraphedNamedAPIResource::from(&obj.version_group),
        }
    }
}

impl From<PokemonHeldItem> for GraphedPokemonHeldItem {
    fn from(obj: PokemonHeldItem) -> Self {
        GraphedPokemonHeldItem {
            item: GraphedNamedAPIResource::from(&obj.item),
            version_details: conv_vector(&obj.version_details),
        }
    }
}

impl From<PokemonMove> for GraphedPokemonMove {
    fn from(obj: PokemonMove) -> Self {
        GraphedPokemonMove {
            move_: GraphedNamedAPIResource::from(&obj.move_),
            version_group_details: conv_vector(&obj.version_group_details),
        }
    }
}

impl From<PalParkEncounterArea> for GraphedPalParkEncounterArea {
    fn from(obj: PalParkEncounterArea) -> Self {
        GraphedPalParkEncounterArea {
            base_score: obj.base_score.to_string(),
            rate: obj.rate.to_string(),
            area: GraphedNamedAPIResource::from(&obj.area),
        }
    }
}

impl From<Genus> for GraphedGenus {
    fn from(obj: Genus) -> Self {
        GraphedGenus {
            genus: obj.genus,
            language: GraphedNamedAPIResource::from(&obj.language),
        }
    }
}

impl From<PokemonSpeciesVariety> for GraphedPokemonSpeciesVariety {
    fn from(obj: PokemonSpeciesVariety) -> Self {
        GraphedPokemonSpeciesVariety {
            is_default: obj.is_default,
            pokemon: GraphedNamedAPIResource::from(&obj.pokemon),
        }
    }
}

impl From<PokemonSpeciesDexEntry> for GraphedPokemonSpeciesDexEntry {
    fn from(obj: PokemonSpeciesDexEntry) -> Self {
        GraphedPokemonSpeciesDexEntry {
            entry_number: obj.entry_number.to_string(),
            pokedex: GraphedNamedAPIResource::from(&obj.pokedex),
        }
    }
}

impl From<PokemonAbility> for GraphedPokemonAbility {
    fn from(obj: PokemonAbility) -> Self {
        GraphedPokemonAbility {
            is_hidden: obj.is_hidden,
            slot: obj.slot.to_string(),
            ability: GraphedNamedAPIResource::from(&obj.ability),
        }
    }
}

impl From<PokemonStat> for GraphedPokemonStat {
    fn from(obj: PokemonStat) -> Self {
        GraphedPokemonStat {
            base_stat: obj.base_stat.to_string(),
            effort: obj.effort.to_string(),
            stat: GraphedNamedAPIResource::from(&obj.stat),
        }
    }
}

impl From<PokemonType> for GraphedPokemonType {
    fn from(obj: PokemonType) -> Self {
        GraphedPokemonType {
            slot: obj.slot.to_string(),
            type_: GraphedNamedAPIResource::from(&obj.type_),
        }
    }
}

impl From<PokemonForm> for GraphedPokemonForm {
    fn from(obj: PokemonForm) -> Self {
        GraphedPokemonForm {
            id: obj.id.to_string(),
            name: obj.name,
            order: obj.order.to_string(),
            form_order: obj.form_order.to_string(),
            is_default: obj.is_default,
            is_battle_only: obj.is_battle_only,
            is_mega: obj.is_mega,
            form_name: obj.form_name,
            pokemon: GraphedNamedAPIResource::from(&obj.pokemon),
            sprites: GraphedPokemonFormSprites::from(obj.sprites),
            version_group: GraphedNamedAPIResource::from(&obj.version_group),
            names: conv_vector(&obj.names),
            form_names: conv_vector(&obj.form_names),
        }
    }
}

impl From<PokemonSprites> for GraphedPokemonSprites {
    fn from(obj: PokemonSprites) -> Self {
        GraphedPokemonSprites {
            back_default: obj.back_default,
            back_female: obj.back_female,
            back_shiny: obj.back_shiny,
            back_shiny_female: obj.back_shiny_female,
            front_default: obj.front_default,
            front_female: obj.front_female,
            front_shiny: obj.front_shiny,
            front_shiny_female: obj.front_shiny_female,
            other: GraphedOtherSprites::from(obj.other),
            versions: GraphedVersionSprites::from(obj.versions),
        }
    }
}

impl From<OtherSprites> for GraphedOtherSprites {
    fn from(obj: OtherSprites) -> Self {
        GraphedOtherSprites {
            dream_world: GraphedDreamWorldSprites::from(obj.dream_world),
            official_artwork: GraphedOfficialArtWorkSprites::from(obj.official_artwork),
        }
    }
}

impl From<DreamWorldSprites> for GraphedDreamWorldSprites {
    fn from(obj: DreamWorldSprites) -> Self {
        GraphedDreamWorldSprites {
            front_default: obj.front_default,
            front_female: obj.front_female,
        }
    }
}

impl From<OfficialArtWorkSprites> for GraphedOfficialArtWorkSprites {
    fn from(obj: OfficialArtWorkSprites) -> Self {
        GraphedOfficialArtWorkSprites {
            front_default: obj.front_default,
        }
    }
}

impl From<VersionSprites> for GraphedVersionSprites {
    fn from(obj: VersionSprites) -> Self {
        GraphedVersionSprites {
            generation_i: GraphedGeneration1Sprites::from(obj.generation_i),
            generation_ii: GraphedGeneration2Sprites::from(obj.generation_ii),
            generation_iii: GraphedGeneration3Sprites::from(obj.generation_iii),
            generation_iv: GraphedGeneration4Sprites::from(obj.generation_iv),
            generation_v: GraphedGeneration5Sprites::from(obj.generation_v),
            generation_vi: GraphedGeneration6Sprites::from(obj.generation_vi),
            generation_vii: GraphedGeneration7Sprites::from(obj.generation_vii),
            generation_viii: GraphedGeneration8Sprites::from(obj.generation_viii),
        }
    }
}

impl From<Generation1Sprites> for GraphedGeneration1Sprites {
    fn from(obj: Generation1Sprites) -> Self {
        GraphedGeneration1Sprites {
            red_blue: GraphedVersionSidesSprites::from(obj.red_blue),
            yellow: GraphedVersionSidesSprites::from(obj.yellow),
        }
    }
}

impl From<VersionSidesSprites> for GraphedVersionSidesSprites {
    fn from(obj: VersionSidesSprites) -> Self {
        GraphedVersionSidesSprites {
            back_default: obj.back_default,
            back_female: obj.back_female,
            back_gray: obj.back_gray,
            back_shiny: obj.back_shiny,
            back_shiny_female: obj.back_shiny_female,
            front_default: obj.front_default,
            front_female: obj.front_female,
            front_gray: obj.front_gray,
            front_shiny: obj.front_shiny,
            front_shiny_female: obj.front_shiny_female,
        }
    }
}

impl From<Generation2Sprites> for GraphedGeneration2Sprites {
    fn from(obj: Generation2Sprites) -> Self {
        GraphedGeneration2Sprites {
            crystal: GraphedVersionSidesSprites::from(obj.crystal),
            gold: GraphedVersionSidesSprites::from(obj.gold),
            silver: GraphedVersionSidesSprites::from(obj.silver),
        }
    }
}

impl From<Generation3Sprites> for GraphedGeneration3Sprites {
    fn from(obj: Generation3Sprites) -> Self {
        GraphedGeneration3Sprites {
            emerald: GraphedVersionSidesSprites::from(obj.emerald),
            firered_leafgreen: GraphedVersionSidesSprites::from(obj.firered_leafgreen),
            ruby_sapphire: GraphedVersionSidesSprites::from(obj.ruby_sapphire),
        }
    }
}

impl From<Generation4Sprites> for GraphedGeneration4Sprites {
    fn from(obj: Generation4Sprites) -> Self {
        GraphedGeneration4Sprites {
            diamond_pearl: GraphedVersionSidesSprites::from(obj.diamond_pearl),
            heartgold_soulsilver: GraphedVersionSidesSprites::from(obj.heartgold_soulsilver),
            platinum: GraphedVersionSidesSprites::from(obj.platinum),
        }
    }
}

impl From<Generation5Sprites> for GraphedGeneration5Sprites {
    fn from(obj: Generation5Sprites) -> Self {
        GraphedGeneration5Sprites {
            black_white: GraphedWithAnimatedVersionSidesSprites::from(obj.black_white),
        }
    }
}

impl From<WithAnimatedVersionSidesSprites> for GraphedWithAnimatedVersionSidesSprites {
    fn from(obj: WithAnimatedVersionSidesSprites) -> Self {
        GraphedWithAnimatedVersionSidesSprites {
            animated: GraphedVersionSidesSprites::from(obj.animated),
            back_default: obj.back_default,
            back_female: obj.back_female,
            back_gray: obj.back_gray,
            back_shiny: obj.back_shiny,
            back_shiny_female: obj.back_shiny_female,
            front_default: obj.front_default,
            front_female: obj.front_female,
            front_gray: obj.front_gray,
            front_shiny: obj.front_shiny,
            front_shiny_female: obj.front_shiny_female,
        }
    }
}

impl From<Generation6Sprites> for GraphedGeneration6Sprites {
    fn from(obj: Generation6Sprites) -> Self {
        GraphedGeneration6Sprites {
            omegaruby_alphasapphire: GraphedVersionSidesSprites::from(obj.omegaruby_alphasapphire),
            x_y: GraphedVersionSidesSprites::from(obj.x_y),
        }
    }
}

impl From<Generation7Sprites> for GraphedGeneration7Sprites {
    fn from(obj: Generation7Sprites) -> Self {
        GraphedGeneration7Sprites {
            icons: GraphedVersionSidesSprites::from(obj.icons),
            ultra_sun_ultra_moon: GraphedVersionSidesSprites::from(obj.ultra_sun_ultra_moon),
        }
    }
}

impl From<Generation8Sprites> for GraphedGeneration8Sprites {
    fn from(obj: Generation8Sprites) -> Self {
        GraphedGeneration8Sprites {
            icons: GraphedVersionSidesSprites::from(obj.icons),
        }
    }
}

impl From<PokemonFormSprites> for GraphedPokemonFormSprites {
    fn from(obj: PokemonFormSprites) -> Self {
        GraphedPokemonFormSprites {
            front_default: obj.front_default,
            back_default: obj.back_default,
            front_shiny: obj.front_shiny,
            back_shiny: obj.back_shiny,
        }
    }
}

impl From<Ability> for GraphedAbility {
    fn from(obj: Ability) -> Self {
        GraphedAbility {
            id: obj.id.to_string(),
            name: obj.name,
            is_main_series: obj.is_main_series,
            generation: GraphedNamedAPIResource::from(&obj.generation),
            names: conv_vector(&obj.names),
            effect_entries: conv_vector(&obj.effect_entries),
            effect_changes: conv_vector(&obj.effect_changes),
            flavor_text_entries: conv_vector(&obj.flavor_text_entries),
            pokemon: conv_vector(&obj.pokemon),
        }
    }
}

impl From<AbilityPokemon> for GraphedAbilityPokemon {
    fn from(obj: AbilityPokemon) -> Self {
        GraphedAbilityPokemon {
            is_hidden: obj.is_hidden,
            slot: obj.slot.to_string(),
            pokemon: GraphedNamedAPIResource::from(&obj.pokemon),
        }
    }
}

impl From<AbilityFlavorText> for GraphedAbilityFlavorText {
    fn from(obj: AbilityFlavorText) -> Self {
        GraphedAbilityFlavorText {
            flavor_text: obj.flavor_text,
            language: GraphedNamedAPIResource::from(&obj.language),
            version_group: GraphedNamedAPIResource::from(&obj.version_group),
        }
    }
}

impl From<AbilityEffectChange> for GraphedAbilityEffectChange {
    fn from(obj: AbilityEffectChange) -> Self {
        GraphedAbilityEffectChange {
            effect_entries: conv_vector(&obj.effect_entries),
            version_group: GraphedNamedAPIResource::from(&obj.version_group),
        }
    }
}
