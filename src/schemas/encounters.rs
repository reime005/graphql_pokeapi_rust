use juniper::{
  graphql_interface, graphql_object, EmptySubscription, FieldResult, GraphQLEnum,
  GraphQLInputObject, GraphQLObject, ScalarValue,
};

use crate::schemas::generic::GraphedName;

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedEncounterMethod {
  pub id: String,
  pub name: String,
  pub order: String,
  pub names: Vec<GraphedName>,
}
