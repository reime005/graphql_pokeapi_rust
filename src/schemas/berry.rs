// extern crate juniper;
use pokerust::{
  Berry, BerryFirmness, BerryFlavor, BerryFlavorMap, FlavorBerryMap, Id, Move, Named,
  NamedAPIResource, NamedAPIResourceList,
};

use crate::schemas::contests::GraphedContestType;
use crate::schemas::generic::*;
use crate::schemas::root::Context;

// use juniper::graphql_interface;

use juniper::{
  graphql_interface, graphql_object, EmptySubscription, FieldResult, GraphQLEnum,
  GraphQLInputObject, GraphQLObject, ScalarValue,
};

/// Berry
#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedBerry {
  pub id: String,
  pub name: String,
  #[graphql(name = "growthTime")]
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
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedBerryFlavorMap {
  pub potency: String,
  pub flavor: GraphedNamedAPIResource,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedBerryFirmness {
  pub id: String,
  pub name: String,
  pub berries: Vec<GraphedNamedAPIResource>,
  pub names: Vec<GraphedName>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedBerryFlavor {
  pub id: String,
  pub name: String,
  pub berries: Vec<GraphedFlavorBerryMap>,
  pub contest_type: GraphedNamedAPIResource,
  pub names: Vec<GraphedName>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedFlavorBerryMap {
  pub potency: String,
  pub berry: GraphedNamedAPIResource,
}

// impl From<&NamedAPIResource<BerryFirmness>> for GraphedBerryFirmness {
//   fn from(berryFirmness: &NamedAPIResource<BerryFirmness>) -> Self {
//     GraphedBerryFirmness {
//       id: berryFirmness.id().to_string(),
//       name: berryFirmness.name().to_string(),
//       berries: berryFirmness
//         .get()
//         .unwrap()
//         .berries
//         .iter()
//         .map(|b| GraphedBerry::from(&b))
//         .collect(),
//       names: berryFirmness
//         .get()
//         .unwrap()
//         .names
//         .iter()
//         .map(|&n| GraphedName::from(n))
//         .collect(),
//     }
//   }
// }

// impl From<&NamedAPIResource<BerryFlavor>> for GraphedBerryFlavor {
//   fn from(berryFlavor: &NamedAPIResource<BerryFlavor>) -> Self {
//     GraphedBerryFlavor {
//       id: berryFlavor.id().to_string(),
//       name: berryFlavor.name().to_string(),
//       berries: berryFlavor
//         .get()
//         .unwrap()
//         .berries
//         .iter()
//         .map(|b| GraphedBerryFlavorMap::from(&b))
//         .collect(),
//       contest_type: berryFlavor.get().unwrap().contest_type,
//     }
//   }
// }

impl From<BerryFlavor> for GraphedBerryFlavor {
  fn from(berryFlavor: BerryFlavor) -> Self {
    GraphedBerryFlavor {
      id: berryFlavor.id.to_string(),
      name: berryFlavor.name,
      contest_type: GraphedNamedAPIResource {
        url: berryFlavor.contest_type.url,
        name: berryFlavor.contest_type.name,
      },
      berries: berryFlavor
        .berries
        .iter()
        .map(|b| GraphedFlavorBerryMap::from(b.clone()))
        .collect(),
      names: berryFlavor
        .names
        .iter()
        .map(|b| GraphedName::from(b.clone()))
        .collect(),
    }
  }
}

impl From<BerryFlavorMap> for GraphedBerryFlavorMap {
  fn from(berryFlavorMap: pokerust::BerryFlavorMap) -> Self {
    GraphedBerryFlavorMap {
      potency: berryFlavorMap.potency.to_string(),
      flavor: GraphedNamedAPIResource {
        url: berryFlavorMap.flavor.url,
        name: berryFlavorMap.flavor.name,
      },
    }
  }
}

impl From<FlavorBerryMap> for GraphedFlavorBerryMap {
  fn from(flavorBerryMap: FlavorBerryMap) -> Self {
    GraphedFlavorBerryMap {
      potency: flavorBerryMap.potency.to_string(),
      berry: GraphedNamedAPIResource {
        url: flavorBerryMap.berry.url,
        name: flavorBerryMap.berry.name,
      },
    }
  }
}

impl From<Berry> for GraphedBerry {
  fn from(berry: Berry) -> Self {
    GraphedBerry {
      id: berry.id.to_string(),
      name: berry.name,
      growth_time: berry.growth_time.to_string(),
      max_harvest: berry.max_harvest.to_string(),
      natural_gift_power: berry.natural_gift_power.to_string(),
      size: berry.size.to_string(),
      smoothness: berry.smoothness.to_string(),
      soil_dryness: berry.soil_dryness.to_string(),
      firmness: GraphedNamedAPIResource {
        url: berry.firmness.url,
        name: berry.firmness.name,
      },
      flavors: berry
        .flavors
        .iter()
        .clone()
        .map(|b| GraphedBerryFlavorMap::from(b.clone()))
        .collect(),
      item: GraphedNamedAPIResource {
        url: berry.item.url,
        name: berry.item.name,
      },
      natural_gift_type: GraphedNamedAPIResource {
        url: berry.natural_gift_type.url,
        name: berry.natural_gift_type.name,
      },
    }
  }
}

// impl From<&NamedAPIResource<Berry>> for GraphedBerry {
//   fn from(berryFirmness: &NamedAPIResource<Berry>) -> Self {
//     GraphedBerry::from(berryFirmness.get().unwrap())
//   }
// }

impl GraphedBerryFlavor {}
impl GraphedBerryFlavorMap {}

// impl GraphedType {}

impl GraphedBerry {
  // fn id(&self) -> &str {
  //   &self.id
  // }

  // fn name(&self) -> &str {
  //   &self.name
  // }

  // fn growth_time(&self) -> &str {
  //   &self.growth_time
  // }

  // fn max_harvest(&self) -> &str {
  //   &self.max_harvest
  // }

  // fn natural_gift_power(&self) -> &str {
  //   &self.natural_gift_power
  // }

  // fn size(&self) -> &str {
  //   &self.size
  // }

  // fn smoothness(&self) -> &str {
  //   &self.smoothness
  // }

  // fn soil_dryness(&self) -> &str {
  //   &self.soil_dryness
  // }
}
