use diesel::{self, prelude::*};
use errors::SResult;
use schema::question_options;
use uuid::Uuid;

#[derive(Identifiable, Queryable)]
pub struct QuestionOption {
    pub id: i32,
    pub uuid: Uuid,
    pub option: String,
    pub test_question_id: i32,
    pub is_correct: Option<bool>,
}

impl QuestionOption {
    pub fn find(id: i32, conn: &PgConnection) -> SResult<QuestionOption> {
        Ok(question_options::table.find(id).get_result(conn)?)
    }

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

    fn delete_multiple(vec: Vec<Uuid>, test_question_id: i32, conn: &PgConnection) -> SResult<()> {
        let delete_count = diesel::delete(
            question_options::table.filter(
                question_options::uuid
                    .eq_any(&vec)
                    .and(question_options::test_question_id.eq(test_question_id)),
            ),
        ).execute(conn)?;

        if delete_count != vec.len() {
            Err(diesel::NotFound)?;
        }
        Ok(())
    }
}

graphql_object!(QuestionOption: () | &self | {
    description: "A type representing an option."

    field id() -> Uuid 
        as "Id of a question."
    {
        self.uuid
    }

    field option() -> &str 
        as "The actual option."
    {
        &self.option
    }

    field is_correct() -> Option<bool> 
        as "Specifies whether an option is correct."
    {
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
    is_correct: Option<Option<bool>>,
}

impl QuestionOptionPatch {
    fn save(self, uuid: Uuid, test_question_id: i32, conn: &PgConnection) -> SResult<()> {
        diesel::update(
            question_options::table.filter(
                question_options::uuid
                    .eq(uuid)
                    .and(question_options::test_question_id.eq(test_question_id)),
            ),
        ).set(self)
        .execute(conn)?;
        Ok(())
    }
}

/// A type to create new option.
#[derive(GraphQLInputObject)]
pub struct QuestionOptionForm {
    /// Option text.
    option: String,
    /// Specifies whether this option is correct.
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

/// A type to update an option for a question.
#[derive(GraphQLInputObject)]
struct QuestionOptionUpdate {
    /// Id of an option.
    id: Uuid,
    /// New option text.
    option: Option<String>,
    /// Specifies whether an option is correct.
    is_correct: Option<Option<bool>>,
}

impl QuestionOptionUpdate {
    fn save_multiple(
        vec: Vec<QuestionOptionUpdate>,
        test_question_id: i32,
        conn: &PgConnection,
    ) -> SResult<()> {
        for opt in vec {
            let opt_patch = QuestionOptionPatch {
                option: opt.option,
                is_correct: opt.is_correct,
            };
            opt_patch.save(opt.id, test_question_id, conn)?;
        }
        Ok(())
    }
}

/// A type to update options for a question.
#[derive(GraphQLInputObject)]
pub struct QuestionOptionsUpdate {
    /// List of new options.
    new: Vec<QuestionOptionForm>,
    /// List of updates for options.
    update: Vec<QuestionOptionUpdate>,
    /// List of ids to delete options.
    remove: Vec<Uuid>,
}

impl QuestionOptionsUpdate {
    pub fn save(self, test_question_id: i32, conn: &PgConnection) -> SResult<()> {
        QuestionOptionForm::save_multiple(self.new, test_question_id, conn)?;
        QuestionOptionUpdate::save_multiple(self.update, test_question_id, conn)?;
        QuestionOption::delete_multiple(self.remove, test_question_id, conn)
    }
}
