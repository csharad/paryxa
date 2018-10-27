use base64;
use warp::{self, Filter, Rejection};

/// A Basic Auth User.
pub struct BasicUser {
    pub username: String,
    pub password: String,
}

/// Reads a `BasicUser` from a Basic Auth Request.
pub fn basic() -> impl Filter<Extract = (BasicUser,), Error = Rejection> + Copy {
    warp::header("Authorization").and_then(parse_authorization)
}

/// Read a Basic Auth Request optionally.
pub fn basic_optional() -> impl Filter<Extract = (Option<BasicUser>,), Error = Rejection> + Copy {
    basic()
        .map(|user| Some(user))
        .or(warp::any().map(|| None))
        .unify()
}

fn parse_authorization(auth: String) -> Result<BasicUser, Rejection> {
    let parts = auth.split_whitespace().collect::<Vec<_>>();
    if parts.len() != 2 {
        Err(warp::reject::bad_request())?;
    }

    let (type_, credentials) = (parts[0], parts[1]);
    if type_ != "Basic" {
        Err(warp::reject::bad_request())?;
    }

    let decoded = base64::decode(&credentials)
        .map_err(|_| warp::reject::bad_request())
        .and_then(|decoded| String::from_utf8(decoded).map_err(|_| warp::reject::bad_request()))?;

    let parts = decoded.split(":").collect::<Vec<_>>();
    if parts.len() != 2 {
        Err(warp::reject::bad_request())?;
    }
    Ok(BasicUser {
        username: parts[0].to_string(),
        password: parts[1].to_string(),
    })
}
