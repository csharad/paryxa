use graphql::Context;
use juniper::RootNode;
use std::sync::Arc;

pub struct Query;

graphql_object!(Query: Context | &self | {});

pub struct Mutation;

graphql_object!(Mutation: Context | &self | {});

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Arc<Schema> {
    Arc::new(Schema::new(Query, Mutation))
}
