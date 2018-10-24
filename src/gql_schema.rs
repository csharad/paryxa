use errors::SResult;
use graphql::Context;
use juniper::RootNode;
use models::user::{LoginUser, User, UserForm, UserInfoUpdate, UserTypeUpdate};
use std::sync::Arc;
use uuid::Uuid;

pub struct Query;

graphql_object!(Query: Context | &self | {
    field users(&executor, query: Option<String>) -> SResult<Vec<User>> {
        Ok(User::find_all(query, &executor.context().conn)?)
    }

    field user(&executor, id: Uuid) -> SResult<User> {
        Ok(User::find_by_uuid(id, &executor.context().conn)?)
    }
});

pub struct Mutation;

graphql_object!(Mutation: Context | &self | {
    field create_user(&executor, user: UserForm) -> SResult<User> {
        Ok(user.save(&executor.context().conn)?)
    }

    field update_user(&executor, id: Uuid, user: UserInfoUpdate) -> SResult<User> {
        Ok(user.save(id, &executor.context().conn)?)
    }

    field update_user_type(&executor, id: Uuid, user_type: UserTypeUpdate) -> SResult<User> {
        Ok(user_type.save(id, &executor.context().conn)?)
    }

    field delete_user(&executor, id: Uuid) -> SResult<User> {
        Ok(User::delete_by_uuid(id, &executor.context().conn)?)
    }

    field login(&executor, user: LoginUser) -> SResult<User> {
        let user = user.try_login(&executor.context().conn)?;
        //TODO: Set cookies
        Ok(user)
    }

    field logout(&executor) -> &str {
        //TODO: Remove cookies
        "Logged out"
    }
});

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Arc<Schema> {
    Arc::new(Schema::new(Query, Mutation))
}
