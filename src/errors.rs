use bcrypt::BcryptError;
use diesel::r2d2::PoolError;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use juniper::{FieldError, IntoFieldError};
use serde_json::{error::Category as SerdeErrorCategory, Error as SerdeJsonError};

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "DieselError: {}", _0)]
    Diesel(#[cause] DieselError),
    #[fail(display = "BcryptError: {}", _0)]
    Bcrypt(#[cause] BcryptError),
    #[fail(display = "PoolError: {}", _0)]
    R2D2(#[cause] PoolError),
    #[fail(display = "Password for the user is incorrect.")]
    IncorrectPassword,
    #[fail(display = "SerdeJsonError: {}", _0)]
    SerdeJson(#[cause] SerdeJsonError),
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

impl From<PoolError> for Error {
    fn from(err: PoolError) -> Error {
        Error::R2D2(err)
    }
}

impl From<SerdeJsonError> for Error {
    fn from(err: SerdeJsonError) -> Error {
        Error::SerdeJson(err)
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
            Error::R2D2(err) => {
                error!("R2D2Error: {:?}", err);
                internal_server_error()
            }
            Error::SerdeJson(err) => match err.classify() {
                SerdeErrorCategory::Io => {
                    error!("SerdeError::Io: {:?}", err);
                    internal_server_error()
                }
                SerdeErrorCategory::Syntax => serde_error(&err, "INVALID_JSON"),
                SerdeErrorCategory::Data => serde_error(&err, "INVALID_DATA"),
                SerdeErrorCategory::Eof => serde_error(&err, "INVALID_JSON"),
            },
            Error::IncorrectPassword => FieldError::new(
                "Given password was incorrect.",
                graphql_value!({
                    "kind": "INCORRECT_PASSWORD"
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

fn serde_error(err: &SerdeJsonError, type_: &'static str) -> FieldError {
    let (line, column) = (err.line() as i32, err.column() as i32);
    FieldError::new(
        "Could not serialize/deserialize data to/from JSON.",
        graphql_value!({
            "kind": type_,
            "line": line,
            "column": column
        }),
    )
}

pub type SResult<T> = Result<T, Error>;
