use chrono::{Duration, NaiveDateTime, Utc};
use diesel::{self, prelude::*};
use errors::SResult;
use models::test_paper::TestPaper;
use schema::test_schedules;
use uuid::Uuid;

#[derive(Identifiable, Queryable)]
pub struct TestSchedule {
    pub id: i32,
    pub uuid: Uuid,
    pub test_paper_id: i32,
    pub time: NaiveDateTime,
    pub duration: i32,
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

    pub fn find_by_uuid(uuid: Uuid, conn: &PgConnection) -> SResult<TestSchedule> {
        Ok(test_schedules::table.filter(test_schedules::uuid.eq(uuid)).get_result(conn)?)
    }

    pub fn delete_by_uuid(uuid: Uuid, conn: &PgConnection) -> SResult<TestSchedule> {
        Ok(
            diesel::delete(test_schedules::table.filter(test_schedules::uuid.eq(uuid)))
                .get_result(conn)?,
        )
    }
}

graphql_object!(TestSchedule: () |&self| {
    description: "A type representing a test schedule."

    field id() -> Uuid 
        as "Id of a test schedule."
    {
        self.uuid
    }

    field time() -> &NaiveDateTime 
        as "Time at which a test will start."
    {
        &self.time
    }

    field duration() -> i32 
        as "Duration for which a test will last."
    {
        self.duration
    }

    field is_happening() -> bool 
        as "Specifies whether a test is currently happening."
    {
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

/// A type to create new schedule for test.
#[derive(GraphQLInputObject)]
pub struct TestScheduleForm {
    /// Id of a test paper.
    test_paper_id: Uuid,
    /// Time at which the test will start.
    time: NaiveDateTime,
    /// Duration of the test.
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

/// A type to update the test schedule.
#[derive(GraphQLInputObject)]
pub struct TestScheduleUpdate {
    /// Id of a test schedule.
    id: Uuid,
    /// New time at which the test will start.
    time: Option<NaiveDateTime>,
    /// New duration of the test.
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
