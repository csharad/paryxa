use super::exec_db;
use actix_web::{
    middleware::identity::RequestIdentity, HttpRequest, HttpResponse, Json, Path, Query,
};
use errors::Error;
use futures::Future;
use models::user::{LoginUser, User, UserForm, UserInfoUpdate, UserTypeUpdate};
use uuid::Uuid;
use AppState;

pub fn create_user(
    req: HttpRequest<AppState>,
    user: Json<UserForm>,
) -> impl Future<Item = Json<User>, Error = Error> {
    let user = user.into_inner();
    exec_db(&req, |conn| {
        let saved = user.save(&conn)?;
        Ok(Json(saved))
    })
}

pub fn update_user(
    req: HttpRequest<AppState>,
    path: Path<Uuid>,
    user: Json<UserInfoUpdate>,
) -> impl Future<Item = Json<User>, Error = Error> {
    let user = user.into_inner();
    let uuid = path.into_inner();
    exec_db(&req, move |conn| {
        let saved = user.save(uuid, &conn)?;
        Ok(Json(saved))
    })
}

#[derive(Deserialize)]
pub struct UserSearchQuery {
    query: Option<String>,
}

pub fn get_all_user(
    req: HttpRequest<AppState>,
    query: Query<UserSearchQuery>,
) -> impl Future<Item = Json<Vec<User>>, Error = Error> {
    exec_db(&req, move |conn| {
        let users = User::find_all(query.into_inner().query, &conn)?;
        Ok(Json(users))
    })
}

pub fn get_user(
    req: HttpRequest<AppState>,
    path: Path<Uuid>,
) -> impl Future<Item = Json<User>, Error = Error> {
    let uuid = path.into_inner();
    exec_db(&req, move |conn| {
        let user = User::find_by_uuid(uuid, &conn)?;
        Ok(Json(user))
    })
}

pub fn delete_user(
    req: HttpRequest<AppState>,
    path: Path<Uuid>,
) -> impl Future<Item = Json<User>, Error = Error> {
    let uuid = path.into_inner();
    exec_db(&req, move |conn| {
        let user = User::delete_by_uuid(uuid, &conn)?;
        Ok(Json(user))
    })
}

pub fn update_user_type(
    req: HttpRequest<AppState>,
    path: Path<Uuid>,
    user: Json<UserTypeUpdate>,
) -> impl Future<Item = Json<User>, Error = Error> {
    let user = user.into_inner();
    let uuid = path.into_inner();
    exec_db(&req, move |conn| {
        let user = user.save(uuid, &conn)?;
        Ok(Json(user))
    })
}

pub fn login(
    req: HttpRequest<AppState>,
    user: Json<LoginUser>,
) -> impl Future<Item = Json<User>, Error = Error> {
    let user = user.into_inner();
    exec_db(&req, move |conn| {
        let user = user.try_login(&conn)?;
        Ok(user)
    }).and_then(move |user| {
        req.remember(user.id.to_string());
        Ok(Json(user))
    })
}

pub fn logout(req: &HttpRequest<AppState>) -> HttpResponse {
    req.forget();
    HttpResponse::Ok().finish()
}
