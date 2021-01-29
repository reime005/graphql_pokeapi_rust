use juniper::{
    graphql_interface, graphql_object, EmptySubscription, FieldResult, GraphQLEnum,
    GraphQLInputObject, GraphQLObject, ScalarValue,
};

use pokerust::EncounterMethod;

use crate::schemas::generic::*;

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedEncounterMethod {
    pub id: String,
    pub name: String,
    pub order: String,
    pub names: Vec<GraphedName>,
}

impl From<EncounterMethod> for GraphedEncounterMethod {
    fn from(obj: EncounterMethod) -> Self {
        GraphedEncounterMethod {
            id: obj.id.to_string(),
            name: obj.name,
            order: obj.order.to_string(),
            names: conv_vector(&obj.names),
        }
    }
}
