use super::JoinPatch;
use basic::BasicUser;
use bcrypt;
use db_types::*;
use diesel::{
    self,
    deserialize::{self, FromSql},
    pg::Pg,
    prelude::*,
    serialize::{self, IsNull, Output, ToSql},
    dsl,
};
use errors::{Error, SResult};
use models::{test_attempt::TestAttempt, test_subscription::TestSubscription};
use schema::users;
use std::io::Write;
use uuid::Uuid;
use {Context, AUTH_CACHE};

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
    pub fn find(id: i32, conn: &PgConnection) -> SResult<User> {
        Ok(users::table.find(id).get_result(conn)?)
    }

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

    pub fn exists_any(conn: &PgConnection) -> SResult<bool> {
        Ok(diesel::select(dsl::exists(users::table.select(users::id))).get_result(conn)?)
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

    pub fn is_admin(&self) -> bool {
        match self.type_ {
            UserType::Admin => true,
            _ => false,
        }
    }
}

graphql_object!(User: Context |&self| {
    description: "A type representing a user."

    field id() -> Uuid
         as "Id of a user." 
    {
        self.uuid
    }

    field first_name() -> &Option<String>
         as "First name of a user." 
    {
        &self.first_name
    }

    field last_name() -> &Option<String>
         as "Last name of a user." 
    {
        &self.last_name
    }

    field full_name() -> Option<String> 
        as "Full name of a user. Concatenated name for convenience."
    {
        match (self.first_name.as_ref(), self.last_name.as_ref()) {
            (None, None) => None,
            (Some(first), None) => Some(first.to_string()),
            (None, Some(last)) => Some(last.to_string()),
            (Some(first), Some(last)) => Some(format!("{} {}", first, last))
        }
    }

    field email() -> &str
         as "Email of a user." 
    {
        &self.email
    }

    field gender() -> &Option<Gender>
         as "Gender of a user." 
    {
        &self.gender
    }

    field contact() -> &Option<String>
         as "Contact number of a user." 
    {
        &self.contact
    }

    field type() -> &UserType
         as "Type of a user. It represents the privileges the user has." 
    {
        &self.type_
    }

    field test_subscriptions(&executor) -> SResult<Vec<TestSubscription>> 
        as "Tests a user has subscribed to." 
    {
        let ctx = executor.context();
        ctx.me_only(self.id)?;
        TestSubscription::find_all_for_user(self.id, &ctx.conn)
    }

    field test_rooms(&executor) -> SResult<Vec<TestAttempt>> 
        as "Tests a user has taken." 
    {
        let ctx = executor.context();
        ctx.me_only(self.id)?;
        TestAttempt::find_for_user(self.id, &ctx.conn)
    }
});

/// Gender of a user.
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

/// Type of a user.
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

/// A type to create a new user.
#[derive(GraphQLInputObject)]
pub struct UserForm {
    /// Email of a new user.
    email: String,
    /// Password of a new user.
    password: String,
}

impl UserForm {
    pub fn save(self, conn: &PgConnection) -> SResult<User> {
        let has_users = User::exists_any(conn)?;
        let password = bcrypt::hash(&self.password, bcrypt::DEFAULT_COST)?;
        let new_user = NewUser {
            email: self.email,
            password,
            type_: if has_users { UserType::Normal } else { UserType::Admin },
            ..NewUser::default()
        };
        new_user.save(conn)
    }
}

/// A type to update the basic information of a user.
#[derive(GraphQLInputObject)]
pub struct UserInfoUpdate {
    /// First name of a user.
    first_name: Option<String>,
    /// Whether the first name is null.
    is_first_name_null: Option<bool>,
    /// Last name of a user.
    last_name: Option<String>,
    /// Whether the last name is null.
    is_last_name_null: Option<bool>,
    /// Gender of a user.
    gender: Option<Gender>,
    /// Whether the gender is null.
    is_gender_null: Option<bool>,
    /// Contact number of a user.
    contact: Option<String>,
    /// Whether the contact number is null.
    is_contact_null: Option<bool>,
}

impl UserInfoUpdate {
    pub fn save(self, id: Uuid, conn: &PgConnection) -> SResult<User> {
        let user_patch = UserPatch {
            first_name: self.first_name.join(self.is_first_name_null),
            last_name: self.last_name.join(self.is_last_name_null),
            gender: self.gender.join(self.is_gender_null),
            contact: self.contact.join(self.is_contact_null),
            ..UserPatch::default()
        };
        user_patch.save(id, conn)
    }
}

/// A type to update the credentials of a user.
#[derive(GraphQLInputObject)]
pub struct UserCredentialsUpdate {
    /// Email of a user.
    email: Option<String>,
    /// New password of a user.
    new_password: Option<String>,
    /// Current password of user.
    password: String,
}

impl UserCredentialsUpdate {
    fn hashed_password(&self) -> SResult<Option<String>> {
        if let Some(ref new_password) = self.new_password {
            let new_hash = bcrypt::hash(&new_password, bcrypt::DEFAULT_COST)?;
            Ok(Some(new_hash))
        } else {
            Ok(None)
        }
    }

    pub fn save(self, id: Uuid, conn: &PgConnection) -> SResult<User> {
        // Verify the password regardless of the update.
        let user = User::find_by_uuid(id, conn)?;
        verify_user(user, &self.password)?;

        let password_hash = self.hashed_password()?;
        let user_patch = UserPatch {
            email: self.email,
            password: password_hash,
            ..UserPatch::default()
        };
        let saved = user_patch.save(id, conn)?;

        // Remove the user from the auth cache as we would not want to
        // authenticate user using the old email/password.
        let basic_user = BasicUser {
            username: saved.email.clone(),
            password: self.password,
        };
        let mut cache = AUTH_CACHE.lock().unwrap();
        cache.remove(&basic_user);
        Ok(saved)
    }
}

/// A type to update the user type for a user.
#[derive(GraphQLInputObject)]
pub struct UserTypeUpdate {
    /// Id of an existing user.
    id: Uuid,
    /// Type of a user.
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

pub fn verify_user(user: User, password: &str) -> SResult<User> {
    if bcrypt::verify(password, &user.password)? {
        Ok(user)
    } else {
        Err(Error::IncorrectPassword)
    }
}
