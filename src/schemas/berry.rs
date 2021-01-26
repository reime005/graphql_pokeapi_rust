// extern crate juniper;
use pokerust::{
  Berry, BerryFirmness, BerryFlavor, BerryFlavorMap, FlavorBerryMap, Id, Named, NamedAPIResource,
  NamedAPIResourceList,
};

use crate::schemas::contests::GraphedContestType;
use crate::schemas::generic::GraphedName;
use crate::schemas::root::Context;

// use juniper::graphql_interface;

use juniper::{
  graphql_interface, graphql_object, EmptySubscription, FieldResult, GraphQLEnum,
  GraphQLInputObject, GraphQLObject, ScalarValue,
};

#[graphql_interface]
trait Node {
  fn id(&self) -> &str;
  fn name(&self) -> &str;
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedBerryFirmness {
  pub id: String,
  pub name: String,
  pub berries: Vec<GraphedBerry>,
  pub names: Vec<GraphedName>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedBerryFlavor {
  pub id: String,
  pub name: String,
  pub berries: Vec<GraphedFlavorBerryMap>,
  pub contest_type: GraphedContestType,
  pub names: Vec<GraphedName>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedContextType {
  pub name: String,
  pub url: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedBerryFlavorMap {
  pub potency: String,
  pub flavor: GraphedBerryFlavor,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct GraphedFlavorBerryMap {
  pub potency: String,
  pub berry: GraphedBerry,
}

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
  pub firmness: GraphedBerryFirmness,
  pub flavors: Vec<GraphedBerryFlavorMap>,
  // pub item: GraphedItem,
  pub natural_gift_type: GraphedType,
}

impl From<&NamedAPIResource<BerryFirmness>> for GraphedBerryFirmness {
  fn from(berryFirmness: &NamedAPIResource<BerryFirmness>) -> Self {
    GraphedBerryFirmness {
      id: berryFirmness.id().to_string(),
      name: berryFirmness.name().to_string(),
      berries: berryFirmness
        .get()
        .unwrap()
        .berries
        .iter()
        .map(|&b| GraphedBerry::from(b))
        .collect(),
      names: berryFirmness
        .get()
        .unwrap()
        .names
        .iter()
        .map(|&n| GraphedName::from(n))
        .collect(),
    }
  }
}

impl From<&NamedAPIResource<Berry>> for GraphedBerry {
  fn from(berryFirmness: &NamedAPIResource<Berry>) -> Self {
    GraphedBerry {
      id: berryFirmness.id().to_string(),
      name: berryFirmness.name().to_string(),

    }
  }
}

impl From<&NamedAPIResource<BerryFlavor>> for GraphedBerryFlavor {
  fn from(berryFlavor: &NamedAPIResource<BerryFlavor>) -> Self {
    GraphedBerryFlavor {
      id: berryFlavor.id().to_string(),
      name: berryFlavor.name().to_string(),
      berries: berryFlavor
        .get()
        .unwrap()
        .berries
        .iter()
        .map(|b| GraphedBerryFlavorMap::from(b))
        .collect(),
    }
  }
}

impl From<&pokerust::BerryFlavorMap> for GraphedBerryFlavorMap {
  fn from(berryFlavorMap: &pokerust::BerryFlavorMap) -> Self {
    GraphedBerryFlavorMap {
      potency: berryFlavorMap.potency.to_string(),
      flavor: GraphedBerryFlavor::from(&berryFlavorMap.flavor),
    }
  }
}

impl From<&pokerust::FlavorBerryMap> for GraphedFlavorBerryMap {
  fn from(flavorBerryMap: &pokerust::FlavorBerryMap) -> Self {
    GraphedFlavorBerryMap {
      potency: flavorBerryMap.potency.to_string(),
      berry: GraphedBerry::from(flavorBerryMap.berry),
    }
  }
}

impl From<pokerust::Berry> for GraphedBerry {
  fn from(berry: pokerust::Berry) -> Self {
    GraphedBerry {
      id: berry.id.to_string(),
      name: berry.name,
      growth_time: berry.growth_time.to_string(),
      max_harvest: berry.max_harvest.to_string(),
      natural_gift_power: berry.natural_gift_power.to_string(),
      size: berry.size.to_string(),
      smoothness: berry.smoothness.to_string(),
      soil_dryness: berry.soil_dryness.to_string(),
      firmness: GraphedBerryFirmness::from(&berry.firmness),
      flavors: berry
        .flavors
        .iter()
        .clone()
        .map(|b| GraphedBerryFlavorMap::from(b))
        .collect(),
      // item: GraphedItem::from(berry.item),
      // natural_gift_type: GraphedType::from(berry.natural_gift_type)
    }
  }
}

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
