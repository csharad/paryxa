use schema::test_questions;
use uuid::Uuid;

#[derive(Identifiable, Queryable)]
pub struct TestQuestion {
    id: i32,
    uuid: Uuid,
    question: String,
    test_paper_id: i32,
}

#[derive(Insertable)]
#[table_name = "test_questions"]
pub struct NewTestQuestion {
    question: String,
    test_paper_id: i32,
}

#[derive(AsChangeset)]
#[table_name = "test_questions"]
pub struct TestQuestionPatch {
    question: Option<String>,
}
