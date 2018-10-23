use schema::question_options;
use uuid::Uuid;

#[derive(Identifiable, Queryable)]
pub struct QuestionOption {
    id: i32,
    uuid: Uuid,
    option: String,
    test_question_id: i32,
}

#[derive(Insertable)]
#[table_name = "question_options"]
pub struct NewQuestionOption {
    option: String,
    test_question_id: i32,
}

#[derive(AsChangeset)]
#[table_name = "question_options"]
pub struct QuestionOptionPatch {
    option: Option<String>,
    test_question_id: Option<i32>,
}
