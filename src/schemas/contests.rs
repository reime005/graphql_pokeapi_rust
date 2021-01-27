use juniper::{
  graphql_interface, graphql_object, EmptySubscription, FieldResult, GraphQLEnum,
  GraphQLInputObject, GraphQLObject, ScalarValue,
};

use pokerust::{ContestComboDetail, ContestComboSets, ContestEffect, ContestName, ContestType};

use crate::schemas::berry::GraphedBerryFlavor;
use crate::schemas::encounters::*;
use crate::schemas::games::*;
use crate::schemas::generic::*;
use crate::schemas::root::Context;

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedContestType {
  pub id: String,
  pub name: String,
  pub berry_flavor: GraphedNamedAPIResource,
  pub names: Vec<GraphedContestName>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedContestName {
  pub name: String,
  pub color: String,
  pub language: GraphedNamedAPIResource,
}

impl From<ContestType> for GraphedContestType {
  fn from(a: ContestType) -> Self {
    GraphedContestType {
      id: a.id.to_string(),
      name: a.name,
      berry_flavor: GraphedNamedAPIResource {
        url: a.berry_flavor.url,
        name: a.berry_flavor.name,
      },
      names: a
        .names
        .iter()
        .map(|b| GraphedContestName::from(b.clone()))
        .collect(),
    }
  }
}

impl From<ContestName> for GraphedContestName {
  fn from(a: ContestName) -> Self {
    GraphedContestName {
      name: a.name,
      color: a.color,
      language: GraphedNamedAPIResource {
        url: a.language.url,
        name: a.language.name,
      },
    }
  }
}
