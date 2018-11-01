#![allow(non_camel_case_types)]

#[derive(SqlType, QueryId)]
#[postgres(type_name = "USER_TYPE")]
pub struct User_type;

#[derive(SqlType)]
#[postgres(type_name = "GENDER_TYPE")]
pub struct Gender_type;

#[derive(SqlType)]
#[postgres(type_name = "TEST_TYPE")]
pub struct Test_type;
