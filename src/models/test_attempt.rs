use chrono::NaiveDateTime;
use chrono::Utc;
use diesel::{self, prelude::*};
use errors::SResult;
use models::{question_answer::QuestionAnswer, test_paper::TestPaper, test_schedule::TestSchedule};
use schema::test_attempts;
use uuid::Uuid;
use Context;

#[derive(Identifiable, Queryable)]
pub struct TestAttempt {
    pub id: i32,
    pub uuid: Uuid,
    pub user_id: i32,
    pub test_paper_id: i32,
    pub test_schedule_id: i32,
    pub start_time: NaiveDateTime,
    pub finish_time: Option<NaiveDateTime>,
    pub has_withdrawn: Option<bool>,
}

impl TestAttempt {
    pub fn find_by_uuid_for_user(
        uuid: Uuid,
        user_id: i32,
        conn: &PgConnection,
    ) -> SResult<TestAttempt> {
        Ok(test_attempts::table
            .filter(
                test_attempts::uuid
                    .eq(uuid)
                    .and(test_attempts::user_id.eq(user_id)),
            ).get_result(conn)?)
    }

    pub fn find_for_user(user_id: i32, conn: &PgConnection) -> SResult<Vec<TestAttempt>> {
        Ok(test_attempts::table
            .filter(test_attempts::user_id.eq(user_id))
            .load(conn)?)
    }
}

graphql_object!(TestAttempt: Context | &self | {
    description: "A type representing a test attempt by a user."

    field id() -> Uuid 
        as "Id of a test attempt."
    {
        self.uuid
    }

    field test_paper(&executor) -> SResult<TestPaper> 
        as "Test paper being attempted."
    {
        TestPaper::find(self.test_paper_id, &executor.context().conn)
    }

    field answers(&executor) -> SResult<Vec<QuestionAnswer>> {
        QuestionAnswer::find_all(self.id, &executor.context().conn)
    }

    field test_schedule(&executor) -> SResult<TestSchedule> 
        as "Schedule of a test attempt."
    {
        TestSchedule::find(self.test_schedule_id, &executor.context().conn)
    }

    field start_time() -> &NaiveDateTime 
        as "When was a test started."
    {
        &self.start_time
    }

    field finish_time() -> &Option<NaiveDateTime> 
        as "When was a test finished."
    {
        &self.finish_time
    }

    field has_withdrawn() -> Option<bool> 
        as "Specified whether a test was withdrawn."
    {
        self.has_withdrawn
    }
});

#[derive(Insertable)]
#[table_name = "test_attempts"]
struct NewTestAttempt {
    user_id: i32,
    test_paper_id: i32,
    test_schedule_id: i32,
    start_time: NaiveDateTime,
    finish_time: Option<NaiveDateTime>,
    has_withdrawn: Option<bool>,
}

impl NewTestAttempt {
    fn save(self, conn: &PgConnection) -> SResult<TestAttempt> {
        Ok(diesel::insert_into(test_attempts::table)
            .values(self)
            .get_result(conn)?)
    }
}

#[derive(AsChangeset)]
#[table_name = "test_attempts"]
pub struct TestAttemptPatch {
    finish_time: Option<NaiveDateTime>,
    has_withdrawn: Option<bool>,
}

impl TestAttemptPatch {
    pub fn leave() -> TestAttemptPatch {
        TestAttemptPatch {
            finish_time: Some(Utc::now().naive_utc()),
            has_withdrawn: Some(true),
        }
    }

    pub fn finish() -> TestAttemptPatch {
        TestAttemptPatch {
            finish_time: Some(Utc::now().naive_utc()),
            has_withdrawn: None,
        }
    }

    pub fn save(
        self,
        test_room_id: Uuid,
        user_id: i32,
        conn: &PgConnection,
    ) -> SResult<TestAttempt> {
        Ok(diesel::update(
            test_attempts::table.filter(
                test_attempts::uuid
                    .eq(test_room_id)
                    .and(test_attempts::user_id.eq(user_id)),
            ),
        ).set(self)
        .get_result(conn)?)
    }
}

/// A type to start a test.
#[derive(GraphQLInputObject)]
pub struct StartTest {
    /// Id of a test paper.
    test_paper_id: Uuid,
    /// Id of a test schedule.
    test_schedule_id: Uuid,
}

impl StartTest {
    pub fn save(self, user_id: i32, conn: &PgConnection) -> SResult<TestAttempt> {
        let test_paper = TestPaper::find_by_uuid(self.test_paper_id, conn)?;
        let test_schedule = TestSchedule::find_by_uuid(self.test_schedule_id, conn)?;
        let new_test = NewTestAttempt {
            user_id,
            test_paper_id: test_paper.id,
            test_schedule_id: test_schedule.id,
            start_time: Utc::now().naive_utc(),
            finish_time: None,
            has_withdrawn: None,
        };
        new_test.save(conn)
    }
}
