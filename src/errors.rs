use bcrypt::BcryptError;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use juniper::{FieldError, IntoFieldError};

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "DieselError: {}", _0)]
    Diesel(#[cause] DieselError),
    #[fail(display = "BcryptError: {}", _0)]
    Bcrypt(#[cause] BcryptError),
    #[fail(display = "Password for the user is incorrect.")]
    IncorrectPassword,
    #[fail(display = "An unauthorized request.")]
    Unauthorized,
    #[fail(display = "Cannot change the user type of the last admin.")]
    LastAdmin,
}

impl From<DieselError> for Error {
    fn from(err: DieselError) -> Error {
        Error::Diesel(err)
    }
}

impl From<BcryptError> for Error {
    fn from(err: BcryptError) -> Error {
        Error::Bcrypt(err)
    }
}

impl IntoFieldError for Error {
    fn into_field_error(self) -> FieldError {
        match self {
            Error::Diesel(err) => match err {
                DieselError::NotFound => FieldError::new(
                    "Could not find the requested resource.",
                    graphql_value!({
                        "kind": "NOT_FOUND"
                    }),
                ),
                DieselError::QueryBuilderError(err) => {
                    error!("DieselError::QueryBuilderError: {:?}", err);
                    FieldError::new(
                        "Tried to update a resource without a value.",
                        graphql_value!({
                            "kind": "NULL_UPDATE"
                        }),
                    )
                }
                DieselError::DatabaseError(kind, err) => match kind {
                    DatabaseErrorKind::UniqueViolation => FieldError::new(
                        "Provided value is not unique.",
                        graphql_value!({
                            "kind": "NOT_UNIQUE"
                        }),
                    ),
                    DatabaseErrorKind::ForeignKeyViolation => FieldError::new(
                        "Could not find the requested resource.",
                        graphql_value!({
                            "kind": "NOT_FOUND"
                        }),
                    ),
                    _ => {
                        error!("DieselError::DatabaseError: {:?}", err);
                        internal_server_error()
                    }
                },
                _ => {
                    error!("DieselError: {:?}", err);
                    internal_server_error()
                }
            },
            Error::Bcrypt(err) => {
                error!("BcryptError: {:?}", err);
                internal_server_error()
            }
            Error::IncorrectPassword => FieldError::new(
                "Given password was incorrect.",
                graphql_value!({
                    "kind": "INCORRECT_PASSWORD"
                }),
            ),
            Error::Unauthorized => FieldError::new(
                "This is an unauthorized request.",
                graphql_value!({
                    "kind": "UNAUTHORIZED"
                }),
            ),
            Error::LastAdmin => FieldError::new(
                "Cannot change the user type of the last admin",
                graphql_value!({
                    "kind": "LAST_ADMIN"
                }),
            ),
        }
    }
}

fn internal_server_error() -> FieldError {
    FieldError::new(
        "Something bad happened.",
        graphql_value!({
            "kind": "INTERNAL_SERVER_ERROR"
        }),
    )
}

pub type SResult<T> = Result<T, Error>;
