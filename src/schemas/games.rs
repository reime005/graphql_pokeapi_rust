use juniper::{
  graphql_interface, graphql_object, EmptySubscription, FieldResult, GraphQLEnum,
  GraphQLInputObject, GraphQLObject, ScalarValue,
};

use crate::schemas::contests::GraphedContestType;
use crate::schemas::encounters::*;
use crate::schemas::generic::*;
use crate::schemas::root::Context;

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedVersion {
  pub id: String,
  pub name: String,
  pub names: Vec<GraphedName>,
  pub version_group: GraphedNamedAPIResource,
}

trait Test<Rhs: ?Sized = Self> {

}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedVersionGroup {
  pub id: String,
  pub name: String,
  pub order: String,
  pub generation: GraphedNamedAPIResource,
  pub move_learn_methods: GraphedNamedAPIResource,
  pub pokedexes: GraphedNamedAPIResource,
  pub regions: GraphedNamedAPIResource,
  pub versions: GraphedNamedAPIResource,
}
