use diesel::{self, prelude::*};
use errors::SResult;
use models::question_option::{QuestionOption, QuestionOptionForm, QuestionOptionsUpdate};
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

    fn delete_multiple(vec: Vec<Uuid>, test_paper_id: i32, conn: &PgConnection) -> SResult<()> {
        let delete_count = diesel::delete(
            test_questions::table.filter(
                test_questions::uuid
                    .eq_any(&vec)
                    .and(test_questions::test_paper_id.eq(test_paper_id)),
            ),
        ).execute(conn)?;

        if delete_count != vec.len() {
            Err(diesel::NotFound)?;
        }
        Ok(())
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

impl TestQuestionPatch {
    fn save(self, uuid: Uuid, test_paper_id: i32, conn: &PgConnection) -> SResult<i32> {
        let id = diesel::update(
            test_questions::table.filter(
                test_questions::uuid
                    .eq(uuid)
                    .and(test_questions::test_paper_id.eq(test_paper_id)),
            ),
        ).set(self)
        .returning(test_questions::id)
        .get_result(conn)?;
        Ok(id)
    }

    fn save_or_find(self, uuid: Uuid, test_paper_id: i32, conn: &PgConnection) -> SResult<i32> {
        if self.question.is_some() {
            self.save(uuid, test_paper_id, conn)
        } else {
            Ok(TestQuestion::find_by_uuid_for_test_paper(uuid, test_paper_id, conn)?.id)
        }
    }
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

#[derive(GraphQLInputObject)]
struct TestQuestionUpdate {
    id: Uuid,
    question: Option<String>,
    options: QuestionOptionsUpdate,
}

impl TestQuestionUpdate {
    fn save_multiple(
        vec: Vec<TestQuestionUpdate>,
        test_paper_id: i32,
        conn: &PgConnection,
    ) -> SResult<()> {
        for quest in vec {
            let quest_patch = TestQuestionPatch {
                question: quest.question,
            };
            let question_id = quest_patch.save_or_find(quest.id, test_paper_id, conn)?;
            quest.options.save(question_id, conn)?;
        }
        Ok(())
    }
}

#[derive(GraphQLInputObject)]
pub struct TestQuestionsUpdate {
    new: Vec<TestQuestionForm>,
    update: Vec<TestQuestionUpdate>,
    remove: Vec<Uuid>,
}

impl TestQuestionsUpdate {
    pub fn save(self, test_paper_id: i32, conn: &PgConnection) -> SResult<()> {
        TestQuestionForm::save_multiple(self.new, test_paper_id, conn)?;
        TestQuestionUpdate::save_multiple(self.update, test_paper_id, conn)?;
        TestQuestion::delete_multiple(self.remove, test_paper_id, conn)?;
        Ok(())
    }
}
