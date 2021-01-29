use std::{borrow::Borrow, convert::TryInto};

use juniper::{
    graphql_interface, graphql_object, EmptySubscription, FieldResult, GraphQLEnum,
    GraphQLInputObject, GraphQLObject, ScalarValue,
};

use pokerust::*;

use crate::schemas::berry::GraphedBerryFlavor;
use crate::schemas::contests::GraphedContestType;
use crate::schemas::encounters::*;
use crate::schemas::games::*;
use crate::schemas::generic::*;
use crate::schemas::root::Context;

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedEvolutionChain {
    pub id: String,
    pub baby_trigger_item: Option<GraphedNamedAPIResource>,
    pub chain: GraphedChainLink,
}

impl From<EvolutionChain> for GraphedEvolutionChain {
    fn from(obj: EvolutionChain) -> Self {
        GraphedEvolutionChain {
            id: obj.id.to_string(),
            baby_trigger_item: match obj.baby_trigger_item {
                Some(val) => Some(GraphedNamedAPIResource::from(&val)),
                None => None,
            },
            chain: GraphedChainLink::from(obj.chain),
        }
    }
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedChainLink {
    pub is_baby: bool,
    pub species: GraphedNamedAPIResource,
    pub evolution_details: Vec<GraphedEvolutionDetail>,
    pub evolves_to: Vec<GraphedChainLink>,
}

impl From<ChainLink> for GraphedChainLink {
    fn from(obj: ChainLink) -> Self {
        GraphedChainLink {
            is_baby: obj.is_baby,
            evolves_to: conv_vector(&obj.evolves_to),
            species: GraphedNamedAPIResource::from(&obj.species),
            evolution_details: conv_vector(&obj.evolution_details),
        }
    }
}

#[derive(GraphQLObject)]
#[graphql(description = "TODO")]
pub struct GraphedEvolutionDetail {
    pub item: Option<GraphedNamedAPIResource>,
    pub trigger: GraphedNamedAPIResource,
    pub gender: Option<String>,
    pub held_item: Option<GraphedNamedAPIResource>,
    pub known_move: Option<GraphedNamedAPIResource>,
    pub known_move_type: Option<GraphedNamedAPIResource>,
    pub location: Option<GraphedNamedAPIResource>,
    pub min_level: Option<String>,
    pub min_happiness: Option<String>,
    pub min_beauty: Option<String>,
    pub min_affection: Option<String>,
    pub needs_overworld_rain: bool,
    pub party_species: Option<GraphedNamedAPIResource>,
    pub party_type: Option<GraphedNamedAPIResource>,
    pub relative_physical_stats: Option<String>,
    pub time_of_day: String,
    pub trade_species: Option<GraphedNamedAPIResource>,
    pub turn_upside_down: bool,
}

impl From<EvolutionDetail> for GraphedEvolutionDetail {
    fn from(obj: EvolutionDetail) -> Self {
        GraphedEvolutionDetail {
            item: match obj.item {
                Some(val) => Some(GraphedNamedAPIResource::from(&val)),
                None => None,
            },
            trigger: GraphedNamedAPIResource::from(&obj.trigger),
            gender: match obj.gender {
                Some(val) => Some(val.to_string()),
                None => None,
            },
            held_item: match obj.held_item {
                Some(val) => Some(GraphedNamedAPIResource::from(&val)),
                None => None,
            },
            known_move: match obj.known_move {
                Some(val) => Some(GraphedNamedAPIResource::from(&val)),
                None => None,
            },
            known_move_type: match obj.known_move_type {
                Some(val) => Some(GraphedNamedAPIResource::from(&val)),
                None => None,
            },
            location: match obj.location {
                Some(val) => Some(GraphedNamedAPIResource::from(&val)),
                None => None,
            },
            min_level: match obj.min_level {
                Some(val) => Some(val.to_string()),
                None => None,
            },
            min_happiness: match obj.min_happiness {
                Some(val) => Some(val.to_string()),
                None => None,
            },
            min_beauty: match obj.min_beauty {
                Some(val) => Some(val.to_string()),
                None => None,
            },
            min_affection: match obj.min_affection {
                Some(val) => Some(val.to_string()),
                None => None,
            },
            needs_overworld_rain: obj.needs_overworld_rain,
            party_species: match obj.party_species {
                Some(val) => Some(GraphedNamedAPIResource::from(&val)),
                None => None,
            },
            party_type: match obj.party_type {
                Some(val) => Some(GraphedNamedAPIResource::from(&val)),
                None => None,
            },
            relative_physical_stats: match obj.relative_physical_stats {
                Some(val) => Some(val.to_string()),
                None => None,
            },
            time_of_day: obj.time_of_day,
            trade_species: match obj.trade_species {
                Some(val) => Some(GraphedNamedAPIResource::from(&val)),
                None => None,
            },
            turn_upside_down: obj.turn_upside_down,
        }
    }
}
