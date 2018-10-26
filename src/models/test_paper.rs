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
    id: i32,
    uuid: Uuid,
    name: String,
    description: Option<String>,
    type_: TestType,
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
}

graphql_object!(TestPaper: Context |&self| {
    field id() -> Uuid {
        self.uuid
    }

    field name() -> &str {
        &self.name
    }

    field description() -> &Option<String> {
        &self.description
    }

    field type() -> &TestType {
        &self.type_
    }

    field questions(&executor) -> SResult<Vec<TestQuestion>> {
        TestQuestion::find_all(self.id, &executor.context().conn)
    }

    field question(&executor, id: Uuid) -> SResult<TestQuestion> {
        TestQuestion::find_by_uuid_for_test_paper(id, self.id, &executor.context().conn)
    }

    field test_schedules(&executor) -> SResult<Vec<TestSchedule>> {
        TestSchedule::find_all_for_test_paper(self.id, &executor.context().conn)
    }
});

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
}

#[derive(GraphQLInputObject)]
pub struct TestPaperForm {
    name: String,
    description: Option<String>,
    type_: TestType,
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

#[derive(GraphQLInputObject)]
pub struct TestPaperUpdate {
    id: Uuid,
    name: Option<String>,
    description: Option<Option<String>>,
    type_: Option<TestType>,
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
            let saved = paper_patch.save(self.id, conn)?;
            self.questions.save(saved.id, conn)?;
            Ok(saved)
        })
    }
}
