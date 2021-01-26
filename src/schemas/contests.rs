#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedContestType {
  pub id: String,
  pub name: String
}
