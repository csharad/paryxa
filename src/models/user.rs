use super::JoinPatch;
use bcrypt;
use db_types::*;
use diesel::{
    self,
    deserialize::{self, FromSql},
    pg::Pg,
    prelude::*,
    serialize::{self, IsNull, Output, ToSql},
};
use errors::{Error, SResult};
use models::{test_room::TestRoom, test_subscription::TestSubscription};
use schema::users;
use std::io::Write;
use uuid::Uuid;
use Context;

#[derive(Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub uuid: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    password: String,
    pub gender: Option<Gender>,
    pub contact: Option<String>,
    pub type_: UserType,
}

impl User {
    pub fn find_by_uuid(uuid: Uuid, conn: &PgConnection) -> SResult<User> {
        let user = users::table.filter(users::uuid.eq(uuid)).get_result(conn)?;
        Ok(user)
    }

    pub fn find_by_email(email: &str, conn: &PgConnection) -> SResult<User> {
        let user = users::table
            .filter(users::email.eq(email))
            .get_result(conn)?;
        Ok(user)
    }

    pub fn find_all(query: Option<String>, conn: &PgConnection) -> SResult<Vec<User>> {
        let users = if let Some(query) = query {
            let like_str = format!("%{}%", query);

            // Currently `ilike` comparisons are not supported for nullable
            // types in diesel.
            users::table
                .filter(
                    users::first_name
                        .like(&like_str)
                        .or(users::last_name.like(&like_str))
                        .or(users::email.ilike(&like_str))
                        .or(users::contact.like(&like_str)),
                ).load(conn)?
        } else {
            users::table.load(conn)?
        };
        Ok(users)
    }

    pub fn delete_by_uuid(uuid: Uuid, conn: &PgConnection) -> SResult<User> {
        let user = diesel::delete(users::table.filter(users::uuid.eq(uuid))).get_result(conn)?;
        Ok(user)
    }
}

graphql_object!(User: Context |&self| {
    field id() -> Uuid {
        self.uuid
    }

    field first_name() -> &Option<String> {
        &self.first_name
    }

    field last_name() -> &Option<String> {
        &self.last_name
    }

    field email() -> &str {
        &self.email
    }

    field gender() -> &Option<Gender> {
        &self.gender
    }

    field contact() -> &Option<String> {
        &self.contact
    }

    field type() -> &UserType {
        &self.type_
    }

    field test_subscriptions(&executor) -> SResult<Vec<TestSubscription>> {
        TestSubscription::find_all_for_user(self.id, &executor.context().conn)
    }

    field test_rooms(&executor) -> SResult<Vec<TestRoom>> {
        TestRoom::find_for_user(self.id, &executor.context().conn)
    }
});

#[derive(Debug, FromSqlRow, AsExpression, GraphQLEnum)]
#[sql_type = "Gender_type"]
pub enum Gender {
    Male,
    Female,
    Other,
}

impl FromSql<Gender_type, Pg> for Gender {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let bytes = not_none!(bytes);
        match bytes {
            b"Male" => Ok(Gender::Male),
            b"Female" => Ok(Gender::Female),
            b"Other" => Ok(Gender::Other),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl ToSql<Gender_type, Pg> for Gender {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match self {
            Gender::Male => out.write_all(b"Male")?,
            Gender::Female => out.write_all(b"Female")?,
            Gender::Other => out.write_all(b"Other")?,
        }
        Ok(IsNull::No)
    }
}

#[derive(Debug, FromSqlRow, AsExpression, GraphQLEnum)]
#[sql_type = "User_type"]
pub enum UserType {
    Admin,
    Normal,
}

impl FromSql<User_type, Pg> for UserType {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let bytes = not_none!(bytes);
        match bytes {
            b"Admin" => Ok(UserType::Admin),
            b"Normal" => Ok(UserType::Normal),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl ToSql<User_type, Pg> for UserType {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match self {
            UserType::Admin => out.write_all(b"Admin")?,
            UserType::Normal => out.write_all(b"Normal")?,
        }
        Ok(IsNull::No)
    }
}

impl Default for UserType {
    fn default() -> UserType {
        UserType::Normal
    }
}

#[derive(Insertable, Default)]
#[table_name = "users"]
struct NewUser {
    first_name: Option<String>,
    last_name: Option<String>,
    email: String,
    password: String,
    gender: Option<Gender>,
    contact: Option<String>,
    type_: UserType,
}

impl NewUser {
    fn save(self, conn: &PgConnection) -> SResult<User> {
        let saved = diesel::insert_into(users::table)
            .values(self)
            .get_result(conn)?;
        Ok(saved)
    }
}

#[derive(Default, AsChangeset)]
#[table_name = "users"]
struct UserPatch {
    first_name: Option<Option<String>>,
    last_name: Option<Option<String>>,
    email: Option<String>,
    password: Option<String>,
    gender: Option<Option<Gender>>,
    contact: Option<Option<String>>,
    type_: Option<UserType>,
}

impl UserPatch {
    fn save(self, uuid: Uuid, conn: &PgConnection) -> SResult<User> {
        let saved = diesel::update(users::table.filter(users::uuid.eq(uuid)))
            .set(self)
            .get_result(conn)?;
        Ok(saved)
    }
}

#[derive(GraphQLInputObject)]
pub struct UserForm {
    email: String,
    password: String,
}

impl UserForm {
    fn hash_password(self) -> SResult<NewUser> {
        let password = bcrypt::hash(&self.password, bcrypt::DEFAULT_COST)?;
        Ok(NewUser {
            email: self.email,
            password,
            ..NewUser::default()
        })
    }

    pub fn save(self, conn: &PgConnection) -> SResult<User> {
        let new_user = self.hash_password()?;
        new_user.save(conn)
    }
}

#[derive(GraphQLInputObject)]
pub struct UserInfoUpdate {
    id: Uuid,
    first_name: Option<String>,
    is_first_name_null: Option<bool>,
    last_name: Option<String>,
    is_last_name_null: Option<bool>,
    gender: Option<Gender>,
    is_gender_null: Option<bool>,
    contact: Option<String>,
    is_contact_null: Option<bool>,
    email: Option<String>,
    password: Option<String>,
}

impl UserInfoUpdate {
    fn hashed_password(&self, uuid: Uuid, conn: &PgConnection) -> SResult<Option<String>> {
        if let Some(ref password) = self.password {
            let user = User::find_by_uuid(uuid, conn)?;
            if bcrypt::verify(password, &user.password)? {
                let new_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
                Ok(Some(new_hash))
            } else {
                Err(Error::IncorrectPassword)
            }
        } else {
            Ok(None)
        }
    }

    pub fn save(self, conn: &PgConnection) -> SResult<User> {
        let password = self.hashed_password(self.id, conn)?;
        let user_patch = UserPatch {
            first_name: self.first_name.join(self.is_first_name_null),
            last_name: self.last_name.join(self.is_last_name_null),
            gender: self.gender.join(self.is_gender_null),
            contact: self.contact.join(self.is_contact_null),
            email: self.email,
            password,
            ..UserPatch::default()
        };
        user_patch.save(self.id, conn)
    }
}

#[derive(GraphQLInputObject)]
pub struct UserTypeUpdate {
    id: Uuid,
    type_: Option<UserType>,
}

impl UserTypeUpdate {
    pub fn save(self, conn: &PgConnection) -> SResult<User> {
        let user_patch = UserPatch {
            type_: self.type_,
            ..UserPatch::default()
        };
        user_patch.save(self.id, conn)
    }
}

#[derive(GraphQLInputObject)]
pub struct LoginUser {
    email: String,
    password: String,
}

impl LoginUser {
    pub fn try_login(self, conn: &PgConnection) -> SResult<User> {
        let user = User::find_by_email(&self.email, conn)?;
        if bcrypt::verify(&self.password, &user.password)? {
            Ok(user)
        } else {
            Err(Error::IncorrectPassword)
        }
    }
}
