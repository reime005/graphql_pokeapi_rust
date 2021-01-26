use juniper::{
  graphql_interface, graphql_object, EmptySubscription, FieldResult, GraphQLEnum,
  GraphQLInputObject, GraphQLObject, ScalarValue,
};

use pokerust::{
  Berry, BerryFirmness, BerryFlavor, BerryFlavorMap, FlavorBerryMap, Id, Language, Name, Named,
  NamedAPIResource, NamedAPIResourceList,
};

use crate::schemas::contests::GraphedContestType;
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
pub struct GraphedName {
  pub name: String,
  pub language: GraphedLanguage,
}

impl From<Name> for GraphedName {
  fn from(name: Name) -> Self {
    GraphedName {
      name: name.name,
      language: GraphedLanguage::from(&name.language),
    }
  }
}

impl From<&NamedAPIResource<Language>> for GraphedLanguage {
  fn from(language: &NamedAPIResource<Language>) -> Self {
    GraphedLanguage {
      id: language.id().to_string(),
      name: language.name().to_string(),
      official: language.get().unwrap().official,
      iso639: language.get().unwrap().iso639,
      iso3166: language.get().unwrap().iso3166,
      names: language
        .get()
        .unwrap()
        .names
        .iter()
        .map(|&n| GraphedName::from(n))
        .collect(),
    }
  }
}
