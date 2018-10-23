use chrono::NaiveDateTime;
use diesel::{data_types::PgInterval, prelude::*};
use errors::SResult;
use schema::test_schedules;
use uuid::Uuid;

#[derive(Identifiable, Queryable)]
pub struct TestSchedule {
    id: i32,
    uuid: Uuid,
    test_paper_id: i32,
    time: NaiveDateTime,
    duration: PgInterval,
}

#[derive(Insertable)]
#[table_name = "test_schedules"]
pub struct NewTestSchedule {
    test_paper_id: i32,
    time: NaiveDateTime,
    duration: PgInterval,
}

#[derive(AsChangeset)]
#[table_name = "test_schedules"]
pub struct TestSchedulePatch {
    time: Option<NaiveDateTime>,
    duration: Option<PgInterval>,
}
