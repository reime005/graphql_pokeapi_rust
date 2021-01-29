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
use crate::schemas::generic::*;
use crate::schemas::root::Context;

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedAPIResourceList {
  pub count: String,
  pub next: Option<String>,
  pub previous: Option<String>,
  pub results: Vec<GraphedAPIResource>,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedNamedAPIResourceList {
  pub count: String,
  pub next: Option<String>,
  pub previous: Option<String>,
  pub results: Vec<GraphedNamedAPIResource>,
}

impl<T> From<NamedAPIResourceList<T>> for GraphedNamedAPIResourceList {
  fn from(name: NamedAPIResourceList<T>) -> Self {
    GraphedNamedAPIResourceList {
      count: name.count.to_string(),
      next: name.next,
      previous: name.previous,
      results: name
        .results
        .iter()
        .map(|r| GraphedNamedAPIResource::from(r))
        .collect(),
    }
  }
}
