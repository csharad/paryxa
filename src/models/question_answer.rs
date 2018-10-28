use diesel::prelude::*;
use models::{question_option::QuestionOption, test_question::TestQuestion};
use schema::question_answers;
use {errors::SResult, Context};

#[derive(Identifiable, Queryable)]
pub struct QuestionAnswer {
    id: i32,
    test_attempt_id: i32,
    test_question_id: i32,
    answered_option: i32,
}

impl QuestionAnswer {
    pub fn find_all(test_attempt_id: i32, conn: &PgConnection) -> SResult<Vec<QuestionAnswer>> {
        Ok(question_answers::table
            .filter(question_answers::test_attempt_id.eq(test_attempt_id))
            .load(conn)?)
    }
}

graphql_object!(QuestionAnswer: Context | &self | {
    field question(&executor) -> SResult<TestQuestion> {
        TestQuestion::find(self.test_question_id, &executor.context().conn)
    }

    field answered_option(&executor) -> SResult<QuestionOption> {
        QuestionOption::find(self.answered_option, &executor.context().conn)
    }
});

#[derive(Insertable)]
#[table_name = "question_answers"]
struct NewQuestionAnswer {
    test_attempt_id: i32,
    test_question_id: i32,
    answered_option: i32,
}

#[derive(AsChangeset)]
#[table_name = "question_answers"]
struct QuestionAnswerPatch {
    answered_option: Option<i32>,
}
