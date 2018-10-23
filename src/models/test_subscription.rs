use schema::test_subscriptions;
use uuid::Uuid;

#[derive(Identifiable, Queryable)]
pub struct TestSubscription {
    id: i32,
    uuid: Uuid,
    user_id: i32,
    test_paper_id: i32,
    test_schedule_id: i32,
}

#[derive(Insertable)]
#[table_name = "test_subscriptions"]
pub struct NewTestSubscription {
    user_id: i32,
    test_paper_id: i32,
    test_schedule_id: i32
}
