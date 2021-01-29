use juniper::{
    graphql_interface, graphql_object, EmptySubscription, FieldResult, GraphQLEnum,
    GraphQLInputObject, GraphQLObject, ScalarValue,
};

use pokerust::{
    APIResource, Berry, BerryFirmness, BerryFlavor, BerryFlavorMap, Description, Effect,
    FlavorBerryMap, FlavorText, Id, Language, Name, Named, NamedAPIResource, NamedAPIResourceList,
    VerboseEffect, VersionGameIndex,
};
use std::marker::PhantomData;

use crate::schemas::contests::GraphedContestType;
use crate::schemas::encounters::*;
use crate::schemas::games::*;
use crate::schemas::root::Context;

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedLanguage {
    pub id: String,
    pub name: String,
    pub official: bool,
    pub iso639: String,
    pub iso3166: String,
    pub names: Vec<GraphedName>,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedAPIResource {
    pub url: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedDescription {
    pub description: String,
    pub language: GraphedNamedAPIResource,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedEffect {
    pub effect: String,
    pub language: GraphedNamedAPIResource,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedEncounter {
    pub min_level: String,
    pub max_level: String,
    // pub condition_values: Vec<GraphedEncounterConditionValue>,
    pub chance: String,
    pub method: GraphedEncounterMethod,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedFlavorText {
    pub flavor_text: String,
    pub language: GraphedNamedAPIResource,
    pub version: Option<GraphedNamedAPIResource>, // sometimes this isn't provided, e.g. https://pokeapi.co/api/v2/contest-effect/9/
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedGenerationGameIndex {
    pub game_index: String,
    pub generation: GraphedNamedAPIResource,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedMachineVersionDetail {
    // pub machine: GraphedMachine,
    pub version_group: GraphedVersionGroup,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedName {
    pub name: String,
    pub language: GraphedNamedAPIResource,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedNamedAPIResource {
    pub name: String,
    pub url: String,
}

impl GraphedNamedAPIResource {}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedVerboseEffect {
    pub effect: String,
    pub short_effect: String,
    pub language: GraphedNamedAPIResource,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedVersionEncounterDetail {
    pub version: GraphedVersion,
    pub max_chance: String,
    pub encounter_details: Vec<GraphedEncounter>,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedVersionGroupFlavorText {
    pub text: String,
    pub language: GraphedNamedAPIResource,
    pub version_group: GraphedNamedAPIResource,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedVersionGameIndex {
    pub game_index: String,
    pub version: GraphedNamedAPIResource,
}

impl From<VersionGameIndex> for GraphedVersionGameIndex {
    fn from(obj: VersionGameIndex) -> Self {
        GraphedVersionGameIndex {
            game_index: obj.game_index.to_string(),
            version: GraphedNamedAPIResource::from(&obj.version),
        }
    }
}

impl From<Description> for GraphedDescription {
    fn from(obj: Description) -> Self {
        GraphedDescription {
            description: obj.description,
            language: GraphedNamedAPIResource::from(&obj.language),
        }
    }
}

impl From<FlavorText> for GraphedFlavorText {
    fn from(obj: FlavorText) -> Self {
        GraphedFlavorText {
            flavor_text: obj.flavor_text,
            language: GraphedNamedAPIResource::from(&obj.language),
            version: match obj.version {
                Some(val) => Some(GraphedNamedAPIResource::from(&val)),
                None => None,
            },
        }
    }
}

impl<T> From<&APIResource<T>> for GraphedAPIResource {
    fn from(name: &APIResource<T>) -> Self {
        GraphedAPIResource {
            url: String::from(&name.url),
        }
    }
}

impl<T> From<NamedAPIResource<T>> for GraphedNamedAPIResource {
    fn from(name: NamedAPIResource<T>) -> Self {
        GraphedNamedAPIResource {
            name: name.name,
            url: name.url
        }
    }
}

impl<T> From<&NamedAPIResource<T>> for GraphedNamedAPIResource {
    fn from(name: &NamedAPIResource<T>) -> Self {
        GraphedNamedAPIResource {
            name: String::from(&name.name),
            url: String::from(&name.url),
        }
    }
}

// impl<T> From<NamedAPIResource<T>> for GraphedNamedAPIResource {
//   fn from(obj: NamedAPIResource<T>) -> Self {
//       GraphedNamedAPIResource {
//           name: obj.name,
//           url: obj.url,
//       }
//   }

impl From<Name> for GraphedName {
    fn from(name: Name) -> Self {
        GraphedName {
            name: name.name,
            language: GraphedNamedAPIResource {
                name: name.language.name,
                url: name.language.url,
            },
        }
    }
}

impl From<Language> for GraphedLanguage {
    fn from(obj: Language) -> Self {
        GraphedLanguage {
            id: obj.id.to_string(),
            name: obj.name,
            official: obj.official,
            iso639: obj.iso639,
            iso3166: obj.iso3166,
            names: conv_vector(&obj.names),
        }
    }
}

impl From<VerboseEffect> for GraphedVerboseEffect {
    fn from(obj: VerboseEffect) -> Self {
        GraphedVerboseEffect {
            effect: obj.effect,
            short_effect: obj.short_effect,
            language: GraphedNamedAPIResource::from(&obj.language),
        }
    }
}

impl From<Effect> for GraphedEffect {
    fn from(obj: Effect) -> Self {
        GraphedEffect {
            effect: obj.effect,
            language: GraphedNamedAPIResource::from(&obj.language),
        }
    }
}

pub fn conv_vector<A: Clone, B: From<A>>(obj: &Vec<A>) -> Vec<B> {
    obj.iter().map(|o| B::from(o.clone())).collect()
}
