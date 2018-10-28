use diesel::{self, prelude::*};
use models::{
    question_option::QuestionOption, test_attempt::TestAttempt, test_question::TestQuestion,
};
use schema::question_answers;
use uuid::Uuid;
use {errors::SResult, Context};

#[derive(Identifiable, Queryable)]
pub struct QuestionAnswer {
    id: i32,
    test_attempt_id: i32,
    test_question_id: i32,
    answered_option: i32,
}

impl QuestionAnswer {
    fn find_optionally_for_attempt_and_question(
        test_attempt_id: i32,
        test_question_id: i32,
        conn: &PgConnection,
    ) -> SResult<Option<QuestionAnswer>> {
        Ok(question_answers::table
            .filter(
                question_answers::test_attempt_id
                    .eq(test_attempt_id)
                    .and(question_answers::test_question_id.eq(test_question_id)),
            ).get_result(conn)
            .optional()?)
    }

    pub fn find_all(test_attempt_id: i32, conn: &PgConnection) -> SResult<Vec<QuestionAnswer>> {
        Ok(question_answers::table
            .filter(question_answers::test_attempt_id.eq(test_attempt_id))
            .load(conn)?)
    }
}

graphql_object!(QuestionAnswer: Context | &self | {
    description: "A type representing an answer to a question."

    field question(&executor) -> SResult<TestQuestion> 
        as "Question to which this answer answers."
    {
        TestQuestion::find(self.test_question_id, &executor.context().conn)
    }

    field answered_option(&executor) -> SResult<QuestionOption> 
        as "The selected option which is an answer."
    {
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

impl NewQuestionAnswer {
    fn save(self, conn: &PgConnection) -> SResult<QuestionAnswer> {
        Ok(diesel::insert_into(question_answers::table)
            .values(self)
            .get_result(conn)?)
    }
}

#[derive(AsChangeset)]
#[table_name = "question_answers"]
struct QuestionAnswerPatch {
    answered_option: Option<i32>,
}

impl QuestionAnswerPatch {
    fn save(self, id: i32, conn: &PgConnection) -> SResult<QuestionAnswer> {
        Ok(diesel::update(question_answers::table.find(id))
            .set(self)
            .get_result(conn)?)
    }
}

/// A type to provide an answer to a test question.
#[derive(GraphQLInputObject)]
pub struct ProvideAnswer {
    /// Id of a test attempt.
    test_attempt_id: Uuid,
    /// Id of a question.
    test_question_id: Uuid,
    /// Id of an answered option to the question.
    answered_option: Uuid,
}

impl ProvideAnswer {
    pub fn save(self, user_id: i32, conn: &PgConnection) -> SResult<QuestionAnswer> {
        let attempt = TestAttempt::find_by_uuid_for_user(self.test_attempt_id, user_id, conn)?;
        let question = TestQuestion::find_by_uuid_for_test_paper(
            self.test_question_id,
            attempt.test_paper_id,
            conn,
        )?;
        let option = QuestionOption::find_by_uuid_for_test_question(
            self.answered_option,
            question.id,
            conn,
        )?;

        if let Some(existing_answer) =
            QuestionAnswer::find_optionally_for_attempt_and_question(attempt.id, question.id, conn)?
        {
            let update = QuestionAnswerPatch {
                answered_option: Some(option.id),
            };
            update.save(existing_answer.id, conn)
        } else {
            let new_answer = NewQuestionAnswer {
                test_attempt_id: attempt.id,
                test_question_id: question.id,
                answered_option: option.id,
            };
            new_answer.save(conn)
        }
    }
}
