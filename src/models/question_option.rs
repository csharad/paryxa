use diesel::{self, prelude::*};
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

    pub fn find_by_uuid_for_test_question(
        uuid: Uuid,
        test_question_id: i32,
        conn: &PgConnection,
    ) -> SResult<QuestionOption> {
        Ok(question_options::table
            .filter(
                question_options::test_question_id
                    .eq(test_question_id)
                    .and(question_options::uuid.eq(uuid)),
            ).get_result(conn)?)
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
struct NewQuestionOption {
    option: String,
    test_question_id: i32,
    is_correct: Option<bool>,
}

impl NewQuestionOption {
    fn save_multiple(vec: Vec<NewQuestionOption>, conn: &PgConnection) -> SResult<()> {
        diesel::insert_into(question_options::table)
            .values(vec)
            .execute(conn)?;
        Ok(())
    }
}

#[derive(AsChangeset)]
#[table_name = "question_options"]
struct QuestionOptionPatch {
    option: Option<String>,
    test_question_id: Option<i32>,
    is_correct: Option<Option<bool>>,
}

#[derive(GraphQLInputObject)]
pub struct QuestionOptionForm {
    option: String,
    is_correct: Option<bool>,
}

impl QuestionOptionForm {
    pub fn save_multiple(
        vec: Vec<QuestionOptionForm>,
        test_question_id: i32,
        conn: &PgConnection,
    ) -> SResult<()> {
        let new_options: Vec<_> = vec
            .into_iter()
            .map(|form| NewQuestionOption {
                option: form.option,
                test_question_id,
                is_correct: form.is_correct,
            }).collect();

        NewQuestionOption::save_multiple(new_options, conn)
    }
}
