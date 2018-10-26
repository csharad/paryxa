use chrono::{Duration, NaiveDateTime, Utc};
use diesel::{self, prelude::*};
use errors::SResult;
use models::test_paper::TestPaper;
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
struct NewTestSchedule {
    test_paper_id: i32,
    time: NaiveDateTime,
    duration: i32,
}

impl NewTestSchedule {
    fn save(self, conn: &PgConnection) -> SResult<TestSchedule> {
        Ok(diesel::insert_into(test_schedules::table)
            .values(self)
            .get_result(conn)?)
    }
}

#[derive(AsChangeset)]
#[table_name = "test_schedules"]
struct TestSchedulePatch {
    time: Option<NaiveDateTime>,
    duration: Option<i32>,
}

impl TestSchedulePatch {
    fn save(self, uuid: Uuid, conn: &PgConnection) -> SResult<TestSchedule> {
        Ok(
            diesel::update(test_schedules::table.filter(test_schedules::uuid.eq(uuid)))
                .set(self)
                .get_result(conn)?,
        )
    }
}

#[derive(GraphQLInputObject)]
pub struct TestScheduleForm {
    test_paper_id: Uuid,
    time: NaiveDateTime,
    duration: i32,
}

impl TestScheduleForm {
    pub fn save(self, conn: &PgConnection) -> SResult<TestSchedule> {
        let test_paper = TestPaper::find_by_uuid(self.test_paper_id, conn)?;
        let new_schedule = NewTestSchedule {
            test_paper_id: test_paper.id,
            time: self.time,
            duration: self.duration,
        };
        new_schedule.save(conn)
    }
}

#[derive(GraphQLInputObject)]
pub struct TestScheduleUpdate {
    id: Uuid,
    time: Option<NaiveDateTime>,
    duration: Option<i32>,
}

impl TestScheduleUpdate {
    pub fn save(self, conn: &PgConnection) -> SResult<TestSchedule> {
        let schedule_patch = TestSchedulePatch {
            time: self.time,
            duration: self.duration,
        };
        schedule_patch.save(self.id, conn)
    }
}
