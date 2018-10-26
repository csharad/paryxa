use errors::SResult;
use juniper::RootNode;
use models::{
    test_paper::{TestPaper, TestPaperForm, TestPaperUpdate},
    user::{LoginUser, User, UserForm, UserInfoUpdate, UserTypeUpdate},
};
use uuid::Uuid;
use Context;

pub struct Query;

graphql_object!(Query: Context | &self | {
    field users(&executor, query: Option<String>) -> SResult<Vec<User>> {
        User::find_all(query, &executor.context().conn)
    }

    field user(&executor, id: Uuid) -> SResult<User> {
        User::find_by_uuid(id, &executor.context().conn)
    }

    field test_papers(&executor) -> SResult<Vec<TestPaper>> {
        TestPaper::find_all(&executor.context().conn)
    }

    field test_paper(&executor, id: Uuid) -> SResult<TestPaper> {
        TestPaper::find_by_uuid(id, &executor.context().conn)
    }
});

pub struct Mutation;

graphql_object!(Mutation: Context | &self | {
    field create_user(&executor, user: UserForm) -> SResult<User> {
        user.save(&executor.context().conn)
    }

    field update_user(&executor, user: UserInfoUpdate) -> SResult<User> {
        user.save(&executor.context().conn)
    }

    field update_user_type(&executor, user_type: UserTypeUpdate) -> SResult<User> {
        user_type.save(&executor.context().conn)
    }

    field delete_user(&executor, id: Uuid) -> SResult<User> {
        User::delete_by_uuid(id, &executor.context().conn)
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

    field create_test_paper(&executor, test_paper: TestPaperForm) -> SResult<TestPaper> {
        test_paper.save(&executor.context().conn)
    }

    field update_test_paper(&executor, test_paper: TestPaperUpdate) -> SResult<TestPaper> {
        test_paper.save(&executor.context().conn)
    }

    field delete_test_paper(&executor, id: Uuid) -> SResult<TestPaper> {
        TestPaper::delete_by_uuid(id, &executor.context().conn)
    }
});

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}
