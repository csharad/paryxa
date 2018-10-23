use chrono::NaiveDateTime;
use schema::test_rooms;
use uuid::Uuid;

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
