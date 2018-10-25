use chrono::NaiveDateTime;
use diesel::prelude::*;
use errors::SResult;
use models::{test_paper::TestPaper, test_schedule::TestSchedule};
use schema::test_rooms;
use uuid::Uuid;
use Context;

#[derive(Identifiable, Queryable)]
pub struct TestRoom {
    id: i32,
    uuid: Uuid,
    user_id: i32,
    test_paper_id: i32,
    test_schedule_id: i32,
    start_time: NaiveDateTime,
    finish_time: Option<NaiveDateTime>,
    has_withdrawn: Option<bool>,
}

impl TestRoom {
    pub fn find_for_user(user_id: i32, conn: &PgConnection) -> SResult<Vec<TestRoom>> {
        Ok(test_rooms::table
            .filter(test_rooms::user_id.eq(user_id))
            .load(conn)?)
    }
}

graphql_object!(TestRoom: Context | &self | {
    field id() -> Uuid {
        self.uuid
    }

    field test_paper(&executor) -> SResult<TestPaper> {
        TestPaper::find(self.test_paper_id, &executor.context().conn)
    }

    field test_schedule(&executor) -> SResult<TestSchedule> {
        TestSchedule::find(self.test_schedule_id, &executor.context().conn)
    }

    field start_time() -> &NaiveDateTime {
        &self.start_time
    }

    field finish_time() -> &Option<NaiveDateTime> {
        &self.finish_time
    }

    field has_withdrawn() -> Option<bool> {
        self.has_withdrawn
    }
});

#[derive(Insertable)]
#[table_name = "test_rooms"]
pub struct NewTestRoom {
    user_id: i32,
    test_paper_id: i32,
    test_schedule_id: i32,
    start_time: NaiveDateTime,
    finish_time: Option<NaiveDateTime>,
    has_withdrawn: Option<bool>,
}

#[derive(AsChangeset)]
#[table_name = "test_rooms"]
pub struct TestRoomPatch {
    finish_time: Option<NaiveDateTime>,
    has_withdrawn: Option<bool>,
}
