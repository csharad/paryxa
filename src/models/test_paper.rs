use db_types::*;
use diesel::{
    self,
    deserialize::{self, FromSql},
    pg::Pg,
    prelude::*,
    serialize::{self, IsNull, Output, ToSql},
};
use errors::SResult;
use models::{
    test_question::{TestQuestion, TestQuestionForm, TestQuestionsUpdate},
    test_schedule::TestSchedule,
};
use schema::test_papers;
use std::io::Write;
use uuid::Uuid;
use Context;

#[derive(Identifiable, Queryable)]
pub struct TestPaper {
    pub id: i32,
    pub uuid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub type_: TestType,
}

impl TestPaper {
    pub fn find_all(conn: &PgConnection) -> SResult<Vec<TestPaper>> {
        Ok(test_papers::table.load(conn)?)
    }

    pub fn find(id: i32, conn: &PgConnection) -> SResult<TestPaper> {
        Ok(test_papers::table.find(id).get_result(conn)?)
    }

    pub fn find_by_uuid(uuid: Uuid, conn: &PgConnection) -> SResult<TestPaper> {
        Ok(test_papers::table
            .filter(test_papers::uuid.eq(uuid))
            .get_result(conn)?)
    }

    pub fn delete_by_uuid(uuid: Uuid, conn: &PgConnection) -> SResult<TestPaper> {
        Ok(
            diesel::delete(test_papers::table.filter(test_papers::uuid.eq(uuid)))
                .get_result(conn)?,
        )
    }
}

graphql_object!(TestPaper: Context |&self| {
    description: "A type representing a test paper."

    field id() -> Uuid
        as "Id of a test paper."
    {
        self.uuid
    }

    field name() -> &str 
        as "Name of a test paper."
    {
        &self.name
    }

    field description() -> &Option<String> 
        as "Description of a test paper."
    {
        &self.description
    }

    field type() -> &TestType 
        as "Type of a test paper."
    {
        &self.type_
    }

    field questions(&executor) -> SResult<Vec<TestQuestion>> 
        as "Questions of a test paper."
    {
        TestQuestion::find_all(self.id, &executor.context().conn)
    }

    field question(&executor, id: Uuid) -> SResult<TestQuestion> 
        as "Question of a test paper with the given id."
    {
        TestQuestion::find_by_uuid_for_test_paper(id, self.id, &executor.context().conn)
    }

    field test_schedules(&executor) -> SResult<Vec<TestSchedule>> 
        as "Schedules of a test paper."
    {
        TestSchedule::find_all_for_test_paper(self.id, &executor.context().conn)
    }
});

/// Type of a test.
#[derive(Debug, FromSqlRow, AsExpression, GraphQLEnum)]
#[sql_type = "Test_type"]
pub enum TestType {
    Scheduled,
    FreeForm,
}

impl FromSql<Test_type, Pg> for TestType {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let bytes = not_none!(bytes);
        match bytes {
            b"Scheduled" => Ok(TestType::Scheduled),
            b"FreeForm" => Ok(TestType::FreeForm),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl ToSql<Test_type, Pg> for TestType {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match self {
            TestType::Scheduled => out.write_all(b"Scheduled")?,
            TestType::FreeForm => out.write_all(b"FreeForm")?,
        }
        Ok(IsNull::No)
    }
}

#[derive(Insertable)]
#[table_name = "test_papers"]
struct NewTestPaper {
    name: String,
    description: Option<String>,
    type_: TestType,
}

impl NewTestPaper {
    fn save(self, conn: &PgConnection) -> SResult<TestPaper> {
        Ok(diesel::insert_into(test_papers::table)
            .values(self)
            .get_result(conn)?)
    }
}

#[derive(AsChangeset)]
#[table_name = "test_papers"]
struct TestPaperPatch {
    name: Option<String>,
    description: Option<Option<String>>,
    type_: Option<TestType>,
}

impl TestPaperPatch {
    fn save(self, uuid: Uuid, conn: &PgConnection) -> SResult<TestPaper> {
        Ok(
            diesel::update(test_papers::table.filter(test_papers::uuid.eq(uuid)))
                .set(self)
                .get_result(conn)?,
        )
    }

    fn is_none(&self) -> bool {
        match self {
            TestPaperPatch {
                name: None,
                description: None,
                type_: None,
            } => true,
            _ => false,
        }
    }

    fn save_or_find(self, uuid: Uuid, conn: &PgConnection) -> SResult<TestPaper> {
        if self.is_none() {
            TestPaper::find_by_uuid(uuid, conn)
        } else {
            self.save(uuid, conn)
        }
    }
}

/// A type to create new test paper.
#[derive(GraphQLInputObject)]
pub struct TestPaperForm {
    /// Name of a test paper.
    name: String,
    /// Description of a test paper. 
    description: Option<String>,
    /// Type of a test paper.
    type_: TestType,
    /// Questions in this test paper.
    questions: Vec<TestQuestionForm>,
}

impl TestPaperForm {
    pub fn save(self, conn: &PgConnection) -> SResult<TestPaper> {
        conn.transaction(|| {
            let new_paper = NewTestPaper {
                name: self.name,
                description: self.description,
                type_: self.type_,
            };
            let saved_paper = new_paper.save(conn)?;
            TestQuestionForm::save_multiple(self.questions, saved_paper.id, conn)?;
            Ok(saved_paper)
        })
    }
}

/// A type to update test paper.
#[derive(GraphQLInputObject)]
pub struct TestPaperUpdate {
    /// Id of a test paper.
    id: Uuid,
    /// New name of a test paper.
    name: Option<String>,
    /// New description of a test paper.
    description: Option<Option<String>>,
    /// New type of a test paper.
    type_: Option<TestType>,
    /// Value to update questions of this test.
    questions: TestQuestionsUpdate,
}

impl TestPaperUpdate {
    pub fn save(self, conn: &PgConnection) -> SResult<TestPaper> {
        conn.transaction(|| {
            let paper_patch = TestPaperPatch {
                name: self.name,
                description: self.description,
                type_: self.type_,
            };
            let saved = paper_patch.save_or_find(self.id, conn)?;
            self.questions.save(saved.id, conn)?;
            Ok(saved)
        })
    }
}
