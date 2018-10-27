use errors::SResult;
use juniper::RootNode;
use models::{
    test_paper::{TestPaper, TestPaperForm, TestPaperUpdate},
    test_schedule::{TestSchedule, TestScheduleForm, TestScheduleUpdate},
    user::{LoginUser, User, UserForm, UserInfoUpdate, UserTypeUpdate},
};
use uuid::Uuid;
use Context;

pub struct Query;

graphql_object!(Query: Context | &self | {
    field me(&executor) -> SResult<&User> {
        executor.context().auth_user()
    }

    field users(&executor, query: Option<String>) -> SResult<Vec<User>> {
        let ctx = executor.context();
        ctx.admin_only()?;
        User::find_all(query, &ctx.conn)
    }

    field user(&executor, id: Uuid) -> SResult<User> {
        let ctx = executor.context();
        ctx.admin_only()?;
        User::find_by_uuid(id, &ctx.conn)
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

    field update_me(&executor, user: UserInfoUpdate) -> SResult<User> {
        let ctx = executor.context();
        let auth = ctx.auth_user()?;
        user.save(auth.uuid, &ctx.conn)
    }

    field update_user_type(&executor, user_type: UserTypeUpdate) -> SResult<User> {
        let ctx = executor.context();
        ctx.admin_only()?;
        user_type.save(&ctx.conn)
    }

    field delete_me(&executor) -> SResult<User> {
        let ctx = executor.context();
        let user = ctx.auth_user()?;
        User::delete_by_uuid(user.uuid, &ctx.conn)
    }

    field create_test_paper(&executor, test_paper: TestPaperForm) -> SResult<TestPaper> {
        let ctx = executor.context();
        ctx.admin_only()?;
        test_paper.save(&ctx.conn)
    }

    field update_test_paper(&executor, test_paper: TestPaperUpdate) -> SResult<TestPaper> {
        let ctx = executor.context();
        ctx.admin_only()?;
        test_paper.save(&ctx.conn)
    }

    field delete_test_paper(&executor, id: Uuid) -> SResult<TestPaper> {
        let ctx = executor.context();
        ctx.admin_only()?;
        TestPaper::delete_by_uuid(id, &ctx.conn)
    }

    field create_test_schedule(&executor, schedule: TestScheduleForm) -> SResult<TestSchedule> {
        let ctx = executor.context();
        ctx.admin_only()?;
        schedule.save(&ctx.conn)
    }

    field update_test_schedule(&executor, schedule: TestScheduleUpdate) -> SResult<TestSchedule> {
        let ctx = executor.context();
        ctx.admin_only()?;
        schedule.save(&ctx.conn)
    }

    field delete_test_schedule(&executor, id: Uuid) -> SResult<TestSchedule> {
        let ctx = executor.context();
        ctx.admin_only()?;
        TestSchedule::delete_by_uuid(id, &ctx.conn)
    }
});

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}
