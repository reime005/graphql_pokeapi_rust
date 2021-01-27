use juniper::{
  graphql_interface, graphql_object, EmptySubscription, FieldResult, GraphQLEnum,
  GraphQLInputObject, GraphQLObject, ScalarValue,
};

use pokerust::{
  Berry, BerryFirmness, BerryFlavor, BerryFlavorMap, FlavorBerryMap, Id, Language, Name, Named,
  NamedAPIResource, NamedAPIResourceList,
};
use std::marker::PhantomData;

use crate::schemas::contests::GraphedContestType;
use crate::schemas::encounters::*;
use crate::schemas::games::*;
use crate::schemas::root::Context;

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedLanguage {
  pub id: String,
  pub name: String,
  pub official: bool,
  pub iso639: String,
  pub iso3166: String,
  pub names: Vec<GraphedName>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedDescription {
  pub description: String,
  pub language: GraphedLanguage,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedEffect {
  pub effect: String,
  pub language: GraphedLanguage,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedEncounter {
  pub min_level: String,
  pub max_level: String,
  // pub condition_values: Vec<GraphedEncounterConditionValue>,
  pub chance: String,
  pub method: GraphedEncounterMethod,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedFlavorText {
  pub flavor_text: String,
  pub language: GraphedLanguage,
  pub version: Option<GraphedVersion>, // sometimes this isn't provided, e.g. https://pokeapi.co/api/v2/contest-effect/9/
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedGenerationGameIndex {
  pub game_index: String,
  // pub generation: GraphedGeneration,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedMachineVersionDetail {
  // pub machine: GraphedMachine,
  pub version_group: GraphedVersionGroup,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedName {
  pub name: String,
  pub language: GraphedNamedAPIResource,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedNamedAPIResource {
  pub name: String,
  pub url: String,
}

impl GraphedNamedAPIResource {}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedVerboseEffect {
  pub effect: String,
  pub short_effect: String,
  pub language: GraphedLanguage,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedVersionEncounterDetail {
  pub version: GraphedVersion,
  pub max_chance: String,
  pub encounter_details: Vec<GraphedEncounter>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedVersionGroupFlavorText {
  pub text: String,
  pub language: GraphedLanguage,
  // pub version_group: GraphedVersionGroup,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedVersionGameIndex {
  pub game_index: String,
  pub version: GraphedVersion,
}

impl From<Name> for GraphedName {
  fn from(name: Name) -> Self {
    GraphedName {
      name: name.name,
      language: GraphedNamedAPIResource {
        name: name.language.name,
        url: name.language.url
      }
    }
  }
}

// impl From<&NamedAPIResource<Language>> for GraphedLanguage {
//   fn from(language: &NamedAPIResource<Language>) -> Self {
//     GraphedLanguage {
//       id: language.id().to_string(),
//       name: language.name().to_string(),
//       official: language.get().unwrap().official,
//       iso639: language.get().unwrap().iso639,
//       iso3166: language.get().unwrap().iso3166,
//       names: language
//         .get()
//         .unwrap()
//         .names
//         .iter()
//         .map(|n| GraphedName::from(n))
//         .collect(),
//     }
//   }
// }
