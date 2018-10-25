use diesel::prelude::*;
use errors::SResult;
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

impl QuestionOption {
    pub fn find_all(test_question_id: i32, conn: &PgConnection) -> SResult<Vec<QuestionOption>> {
        Ok(question_options::table
            .filter(question_options::test_question_id.eq(test_question_id))
            .load(conn)?)
    }
}

graphql_object!(QuestionOption: () | &self | {
    field id() -> Uuid {
        self.uuid
    }

    field option() -> &str {
        &self.option
    }

    field is_correct() -> Option<bool> {
        self.is_correct
    }
});

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
