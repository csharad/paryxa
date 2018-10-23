use db_types::*;
use diesel::{
    deserialize::{self, FromSql},
    pg::Pg,
    serialize::{self, IsNull, Output, ToSql},
};
use schema::test_papers;
use std::io::Write;
use uuid::Uuid;

#[derive(Identifiable, Queryable)]
pub struct TestPaper {
    id: i32,
    uuid: Uuid,
    name: String,
    description: Option<String>,
    type_: TestType,
}

#[derive(Debug, FromSqlRow, AsExpression)]
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
