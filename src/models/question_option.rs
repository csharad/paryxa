use schema::question_options;
use uuid::Uuid;

#[derive(Identifiable, Queryable)]
pub struct QuestionOption {
    id: i32,
    uuid: Uuid,
    option: String,
    test_question_id: i32,
    is_correct: Option<bool>,
}

#[derive(Insertable)]
#[table_name = "question_options"]
pub struct NewQuestionOption {
    option: String,
    test_question_id: i32,
    is_correct: Option<bool>,
}

#[derive(AsChangeset)]
#[table_name = "question_options"]
pub struct QuestionOptionPatch {
    option: Option<String>,
    test_question_id: Option<i32>,
    is_correct: Option<Option<bool>>,
}
