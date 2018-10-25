use chrono::{Duration, NaiveDateTime, Utc};
use diesel::prelude::*;
use errors::SResult;
use schema::test_schedules;
use uuid::Uuid;

#[derive(Identifiable, Queryable)]
pub struct TestSchedule {
    id: i32,
    uuid: Uuid,
    test_paper_id: i32,
    time: NaiveDateTime,
    duration: i32,
}

impl TestSchedule {
    pub fn find_all_for_test_paper(
        test_paper_id: i32,
        conn: &PgConnection,
    ) -> SResult<Vec<TestSchedule>> {
        Ok(test_schedules::table
            .filter(test_schedules::test_paper_id.eq(test_paper_id))
            .load(conn)?)
    }

    pub fn find(id: i32, conn: &PgConnection) -> SResult<TestSchedule> {
        Ok(test_schedules::table.find(id).get_result(conn)?)
    }
}

graphql_object!(TestSchedule: () |&self| {
    field id() -> Uuid {
        self.uuid
    }

    field time() -> &NaiveDateTime {
        &self.time
    }

    field duration() -> i32 {
        self.duration
    }

    field is_happening() -> bool {
        let now = Utc::now().naive_utc();
        self.time < now && self.time + Duration::seconds(self.duration as i64) > now 
    }
});

#[derive(Insertable)]
#[table_name = "test_schedules"]
pub struct NewTestSchedule {
    test_paper_id: i32,
    time: NaiveDateTime,
    duration: i32,
}

#[derive(AsChangeset)]
#[table_name = "test_schedules"]
pub struct TestSchedulePatch {
    time: Option<NaiveDateTime>,
    duration: Option<i32>,
}
