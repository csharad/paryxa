use diesel::{self, prelude::*, dsl};
use errors::SResult;
use models::question_option::{QuestionOption, QuestionOptionForm, QuestionOptionsUpdate};
use schema::test_questions;
use uuid::Uuid;
use Context;

#[derive(Identifiable, Queryable)]
pub struct TestQuestion {
    pub id: i32,
    pub uuid: Uuid,
    pub question: String,
    pub test_paper_id: i32,
}

impl TestQuestion {
    pub fn find(id: i32, conn: &PgConnection) -> SResult<TestQuestion> {
        Ok(test_questions::table.find(id).get_result(conn)?)
    }

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

    pub fn count_questions_for_paper(test_paper_id: i32, conn: &PgConnection) -> SResult<i32> {
        let count: i64 = test_questions::table
            .filter(test_questions::test_paper_id.eq(test_paper_id))
            .select(dsl::count_star())
            .get_result(conn)?;
        Ok(count as i32)
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
    description: "A type representing a test question."

    field id() -> Uuid 
        as "Id of a question."
    {
        self.uuid
    }

    field question() -> &str 
        as "The actual question."
    {
        &self.question
    }

    field options(&executor) -> SResult<Vec<QuestionOption>> 
        as "Options of a question."
    {
        QuestionOption::find_all(self.id, &executor.context().conn)
    }

    field option(&executor, id: Uuid) -> SResult<QuestionOption> 
        as "Option of a question with the given id."
    {
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

/// A type to create new test question.
#[derive(GraphQLInputObject)]
pub struct TestQuestionForm {
    /// Question text.
    question: String,
    /// List of options for this question.
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

/// A type to update a test question.
#[derive(GraphQLInputObject)]
struct TestQuestionUpdate {
    /// Id of a test question.
    id: Uuid,
    /// New question text.
    question: Option<String>,
    /// Update type for options.
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

/// A type to update questions.
#[derive(GraphQLInputObject)]
pub struct TestQuestionsUpdate {
    /// List of new questions.
    new: Vec<TestQuestionForm>,
    /// List of updated questions.
    update: Vec<TestQuestionUpdate>,
    /// List of ids to delete older questions.
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
