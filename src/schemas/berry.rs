use pokerust::{
    Berry, BerryFirmness, BerryFlavor, BerryFlavorMap, FlavorBerryMap, Id, Move, Named,
    NamedAPIResource, NamedAPIResourceList,
};

use crate::schemas::contests::GraphedContestType;
use crate::schemas::generic::*;
use crate::schemas::root::Context;

use juniper::{
    graphql_interface, graphql_object, EmptySubscription, FieldResult, GraphQLEnum,
    GraphQLInputObject, GraphQLObject, ScalarValue,
};

/// Berry
#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedBerry {
    pub id: String,
    pub name: String,
    pub growth_time: String,
    pub max_harvest: String,
    pub natural_gift_power: String,
    pub size: String,
    pub smoothness: String,
    pub soil_dryness: String,
    pub firmness: GraphedNamedAPIResource,
    pub flavors: Vec<GraphedBerryFlavorMap>,
    pub item: GraphedNamedAPIResource,
    pub natural_gift_type: GraphedNamedAPIResource,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedBerryFlavorMap {
    pub potency: String,
    pub flavor: GraphedNamedAPIResource,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedBerryFirmness {
    pub id: String,
    pub name: String,
    pub berries: Vec<GraphedNamedAPIResource>,
    pub names: Vec<GraphedName>,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedBerryFlavor {
    pub id: String,
    pub name: String,
    pub berries: Vec<GraphedFlavorBerryMap>,
    pub contest_type: GraphedNamedAPIResource,
    pub names: Vec<GraphedName>,
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedFlavorBerryMap {
    pub potency: String,
    pub berry: GraphedNamedAPIResource,
}

impl From<BerryFlavor> for GraphedBerryFlavor {
    fn from(obj: BerryFlavor) -> Self {
        GraphedBerryFlavor {
            id: obj.id.to_string(),
            name: obj.name,
            contest_type: GraphedNamedAPIResource::from(&obj.contest_type),
            berries: conv_vector(&obj.berries),
            names: conv_vector(&obj.names),
        }
    }
}

impl From<BerryFlavorMap> for GraphedBerryFlavorMap {
    fn from(obj: BerryFlavorMap) -> Self {
        GraphedBerryFlavorMap {
            potency: obj.potency.to_string(),
            flavor: GraphedNamedAPIResource::from(&obj.flavor),
        }
    }
}

impl From<FlavorBerryMap> for GraphedFlavorBerryMap {
    fn from(obj: FlavorBerryMap) -> Self {
        GraphedFlavorBerryMap {
            potency: obj.potency.to_string(),
            berry: GraphedNamedAPIResource::from(&obj.berry),
        }
    }
}

impl From<Berry> for GraphedBerry {
    fn from(obj: Berry) -> Self {
        GraphedBerry {
            id: obj.id.to_string(),
            name: obj.name,
            growth_time: obj.growth_time.to_string(),
            max_harvest: obj.max_harvest.to_string(),
            natural_gift_power: obj.natural_gift_power.to_string(),
            size: obj.size.to_string(),
            smoothness: obj.smoothness.to_string(),
            soil_dryness: obj.soil_dryness.to_string(),
            firmness: GraphedNamedAPIResource::from(&obj.firmness),
            flavors: conv_vector(&obj.flavors),
            item: GraphedNamedAPIResource::from(&obj.item),
            natural_gift_type: GraphedNamedAPIResource::from(&obj.natural_gift_type),
        }
    }
}
