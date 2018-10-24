use juniper::RootNode;
use std::sync::Arc;

pub struct Query;

graphql_object!(Query: () | &self | {
    
});

pub struct Mutation;

graphql_object!(Mutation: () | &self | {

});

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Arc<Schema> {
    Arc::new(Schema::new(Query, Mutation))
}
