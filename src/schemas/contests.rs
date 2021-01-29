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
#[graphql(description = "TODO")]
pub struct GraphedContestType {
    pub id: String,
    pub name: String,
    pub berry_flavor: GraphedNamedAPIResource,
    pub names: Vec<GraphedContestName>,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedContestName {
    pub name: String,
    pub color: String,
    pub language: GraphedNamedAPIResource,
}

impl From<ContestType> for GraphedContestType {
    fn from(obj: ContestType) -> Self {
        GraphedContestType {
            id: obj.id.to_string(),
            name: obj.name,
            berry_flavor: GraphedNamedAPIResource::from(&obj.berry_flavor),
            names: conv_vector(&obj.names),
        }
    }
}

impl From<ContestName> for GraphedContestName {
    fn from(obj: ContestName) -> Self {
        GraphedContestName {
            name: obj.name,
            color: obj.color,
            language: GraphedNamedAPIResource::from(&obj.language),
        }
    }
}
