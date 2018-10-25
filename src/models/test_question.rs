use diesel::prelude::*;
use errors::SResult;
use graphql::Context;
use models::question_option::QuestionOption;
use schema::test_questions;
use uuid::Uuid;

#[derive(Identifiable, Queryable)]
pub struct TestQuestion {
    id: i32,
    uuid: Uuid,
    question: String,
    test_paper_id: i32,
}

impl TestQuestion {
    pub fn find_all(test_paper_id: i32, conn: &PgConnection) -> SResult<Vec<TestQuestion>> {
        Ok(test_questions::table
            .filter(test_questions::test_paper_id.eq(test_paper_id))
            .load(conn)?)
    }
}

graphql_object!(TestQuestion: Context |&self| {
    field id() -> Uuid {
        self.uuid
    }

    field question() -> &str {
        &self.question
    }

    field options(&executor) -> SResult<Vec<QuestionOption>> {
        QuestionOption::find_all(self.id, &executor.context().conn)
    }
});

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
