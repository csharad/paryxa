use errors::SResult;
use juniper::RootNode;
use models::{
    test_paper::{TestPaper, TestPaperForm, TestPaperUpdate},
    test_schedule::{TestSchedule, TestScheduleForm, TestScheduleUpdate},
    user::{User, UserCredentialsUpdate, UserForm, UserInfoUpdate, UserTypeUpdate},
    test_room::{StartTest, TestRoom, TestRoomPatch}
};
use uuid::Uuid;
use Context;

pub struct Query;

graphql_object!(Query: Context | &self | {
    description: "Root query type."

    field me(&executor) -> SResult<&User> 
        as "Gets the current authenticated user." 
    {
        executor.context().auth_user()
    }

    field users(&executor, query: Option<String>) -> SResult<Vec<User>> 
        as "Gets all the users based on the given query." 
    {
        let ctx = executor.context();
        ctx.admin_only()?;
        User::find_all(query, &ctx.conn)
    }

    field user(&executor, id: Uuid) -> SResult<User> 
        as "Gets a user with the given id." 
    {
        let ctx = executor.context();
        ctx.admin_only()?;
        User::find_by_uuid(id, &ctx.conn)
    }

    field test_papers(&executor) -> SResult<Vec<TestPaper>> 
        as "Gets all the test papers." 
    {
        TestPaper::find_all(&executor.context().conn)
    }

    field test_paper(&executor, id: Uuid) -> SResult<TestPaper> 
        as "Gets a test paper with the given id." 
    {
        TestPaper::find_by_uuid(id, &executor.context().conn)
    }
});

pub struct Mutation;

graphql_object!(Mutation: Context | &self | {
    description: "Root mutation type."

    field create_user(&executor, user: UserForm) -> SResult<User> 
        as "Creates a new user."
    {
        user.save(&executor.context().conn)
    }

    field update_me(&executor, user: UserInfoUpdate) -> SResult<User> 
        as "Updates the authenticated user with general information."
    {
        let ctx = executor.context();
        let auth = ctx.auth_user()?;
        user.save(auth.uuid, &ctx.conn)
    }

    field update_my_credentials(&executor, user: UserCredentialsUpdate) -> SResult<User> 
        as "Updates the authenticated user with new credentials."
    {
        let ctx = executor.context();
        let auth = ctx.auth_user()?;
        user.save(auth.uuid, &ctx.conn)
    }

    field update_user_type(&executor, user_type: UserTypeUpdate) -> SResult<User> 
        as "Changes the user type for a user."
    {
        let ctx = executor.context();
        ctx.admin_only()?;
        user_type.save(&ctx.conn)
    }

    field delete_me(&executor) -> SResult<User> 
        as "Deletes the authenticated user."
    {
        let ctx = executor.context();
        let user = ctx.auth_user()?;
        User::delete_by_uuid(user.uuid, &ctx.conn)
    }

    field create_test_paper(&executor, test_paper: TestPaperForm) -> SResult<TestPaper> 
        as "Creates a new test paper."
    {
        let ctx = executor.context();
        ctx.admin_only()?;
        test_paper.save(&ctx.conn)
    }

    field update_test_paper(&executor, test_paper: TestPaperUpdate) -> SResult<TestPaper> 
        as "Updates a test paper."
    {
        let ctx = executor.context();
        ctx.admin_only()?;
        test_paper.save(&ctx.conn)
    }

    field delete_test_paper(&executor, id: Uuid) -> SResult<TestPaper> 
        as "Deletes a test paper with the given id."
    {
        let ctx = executor.context();
        ctx.admin_only()?;
        TestPaper::delete_by_uuid(id, &ctx.conn)
    }

    field create_test_schedule(&executor, schedule: TestScheduleForm) -> SResult<TestSchedule> 
        as "Creates a new test schedule."
    {
        let ctx = executor.context();
        ctx.admin_only()?;
        schedule.save(&ctx.conn)
    }

    field update_test_schedule(&executor, schedule: TestScheduleUpdate) -> SResult<TestSchedule> 
        as "Updates a test schedule."
    {
        let ctx = executor.context();
        ctx.admin_only()?;
        schedule.save(&ctx.conn)
    }

    field delete_test_schedule(&executor, id: Uuid) -> SResult<TestSchedule> 
        as "Deletes a test schedule with the given id."
    {
        let ctx = executor.context();
        ctx.admin_only()?;
        TestSchedule::delete_by_uuid(id, &ctx.conn)
    }

    field start_test(&executor, test: StartTest) -> SResult<TestRoom> 
        as "Start the test for the authenticated user."
    {
        let ctx = executor.context();
        let user = ctx.auth_user()?;
        test.save(user.id, &ctx.conn)
    }

    field leave_test(&executor, test_room_id: Uuid) -> SResult<TestRoom> 
        as "Leave the test in between for the authenticated user."
    {
        let ctx = executor.context();
        let user = ctx.auth_user()?;
        TestRoomPatch::leave().save(test_room_id, user.id, &ctx.conn)
    }

    field finish_test(&executor, test_room_id: Uuid) -> SResult<TestRoom> 
        as "Finish the test for the authenticated user."
    {
        let ctx = executor.context();
        let user = ctx.auth_user()?;
        TestRoomPatch::finish().save(test_room_id, user.id, &ctx.conn)
    }
});

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}
