use db_types::*;
use diesel::{
    deserialize::{self, FromSql},
    pg::Pg,
    prelude::*,
    serialize::{self, IsNull, Output, ToSql},
};
use errors::SResult;
use models::test_question::TestQuestion;
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
pub struct NewTestPaper {
    name: String,
    description: Option<String>,
    type_: TestType,
}

#[derive(AsChangeset)]
#[table_name = "test_papers"]
pub struct TestPaperPatch {
    name: Option<String>,
    description: Option<Option<String>>,
    type_: Option<TestType>,
}
