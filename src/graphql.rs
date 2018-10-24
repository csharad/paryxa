use actix_web::{actix::*, Error, HttpRequest, HttpResponse, Json, State};
use futures::Future;
use gql_schema::Schema;
use juniper::{graphiql::graphiql_source, http::GraphQLRequest};
use serde_json;
use std::sync::Arc;
use AppState;

#[derive(Serialize, Deserialize)]
pub struct GraphQLData(GraphQLRequest);

impl Message for GraphQLData {
    type Result = Result<String, Error>;
}

pub struct GraphQLExecutor {
    schema: Arc<Schema>,
}

impl GraphQLExecutor {
    pub fn new(schema: Arc<Schema>) -> GraphQLExecutor {
        GraphQLExecutor { schema }
    }
}

impl Actor for GraphQLExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<GraphQLData> for GraphQLExecutor {
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: GraphQLData, _: &mut Self::Context) -> Self::Result {
        let res = msg.0.execute(&self.schema, &());
        let res_text = serde_json::to_string(&res)?;
        Ok(res_text)
    }
}

pub fn graphql(
    state: State<AppState>,
    data: Json<GraphQLData>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    state
        .gql_executor
        .send(data.0)
        .from_err()
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(res)),
            Err(_) => Ok(HttpResponse::InternalServerError().finish()),
        })
}

pub fn graphiql(_: &HttpRequest<AppState>) -> HttpResponse {
    let html = graphiql_source("http://localhost:4000/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
