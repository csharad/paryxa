use diesel::{self, prelude::*};
use errors::SResult;
use models::question_option::{QuestionOption, QuestionOptionForm};
use schema::test_questions;
use uuid::Uuid;
use Context;

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

    pub fn find_by_uuid_for_test_paper(
        uuid: Uuid,
        test_paper_id: i32,
        conn: &PgConnection,
    ) -> SResult<TestQuestion> {
        Ok(test_questions::table
            .filter(
                test_questions::test_paper_id
                    .eq(test_paper_id)
                    .and(test_questions::uuid.eq(uuid)),
            ).get_result(conn)?)
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

    field option(&executor, id: Uuid) -> SResult<QuestionOption> {
        QuestionOption::find_by_uuid_for_test_question(id, self.id, &executor.context().conn)
    }
});

#[derive(Insertable)]
#[table_name = "test_questions"]
struct NewTestQuestion {
    question: String,
    test_paper_id: i32,
}

impl NewTestQuestion {
    fn save(self, conn: &PgConnection) -> SResult<i32> {
        Ok(diesel::insert_into(test_questions::table)
            .values(self)
            .returning(test_questions::id)
            .get_result(conn)?)
    }
}

#[derive(AsChangeset)]
#[table_name = "test_questions"]
struct TestQuestionPatch {
    question: Option<String>,
}

#[derive(GraphQLInputObject)]
pub struct TestQuestionForm {
    question: String,
    options: Vec<QuestionOptionForm>,
}

impl TestQuestionForm {
    pub fn save_multiple(
        vec: Vec<TestQuestionForm>,
        test_paper_id: i32,
        conn: &PgConnection,
    ) -> SResult<()> {
        for quest in vec {
            let new_quest = NewTestQuestion {
                question: quest.question,
                test_paper_id,
            };
            let new_id = new_quest.save(conn)?;
            QuestionOptionForm::save_multiple(quest.options, new_id, conn)?;
        }
        Ok(())
    }
}
