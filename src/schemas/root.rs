// use juniper::{FieldError, FieldResult, RootNode};

use juniper::{
    graphql_object, EmptySubscription, FieldResult, GraphQLEnum, GraphQLInputObject, GraphQLObject,
    ScalarValue,
};

use pokerust::{Berry, Endpoint, FromId, FromName, Item, List, NamedAPIResourceList};

use super::berry::GraphedBerry;

pub struct Context {}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[graphql_object(
    // Here we specify the context type for the object.
    // We need to do this in every type that
    // needs access to the context.
    context = Context,
)]
impl QueryRoot {
    fn apiVersion() -> &str {
        "1.0"
    }

    #[graphql(description = "Retrieve a berry information by its name")]
    fn berry_by_name(context: &Context, name: String) -> FieldResult<GraphedBerry> {
        let berry = pokerust::Berry::from_name(&name)?;

        // TODO: flavor map, resource list
        // move to other function
        Ok(GraphedBerry::from(berry))
    }

    #[graphql(description = "Retrieve a berry information by its id")]
    fn berry_by_id(context: &Context, id: String) -> FieldResult<GraphedBerry> {
        let berry = pokerust::Berry::from_id(id.parse()?)?;
        Ok(GraphedBerry::from(berry))
    }

    // #[graphql(description = "Retrieve a berry information by its id")]
    // fn list(context: &Context, next: String, offset: String) -> FieldResult<GraphedBerry> {
    //     let items = pokerust::Item::list(5, 20)?;
    //     Ok(items)
    // }
}

pub struct MutationRoot;

#[graphql_object(
    context = Context,
)]
impl MutationRoot {
    fn apiVersion() -> &str {
        "1.0"
    }
}

pub type Schema = juniper::RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

// pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}
