mod app_state;
mod coffee_type;
mod database;
mod graphql_schema;

use async_graphql::{EmptySubscription, Schema};
use dotenv;
use warp::Filter;

use app_state::AppState;
use graphql_schema::{MutationRoot, QueryRoot};
use rusoto_core::Region;
use rusoto_credential::{EnvironmentProvider, ProvideAwsCredentials};
use rusoto_dynamodb::DynamoDbClient;

mod filters {
    use super::MutationRoot;
    use super::QueryRoot;
    use async_graphql::{EmptySubscription, Schema};
    use warp::Filter;

    pub fn health() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path!("health").and(warp::get()).map(|| Ok("Ok"))
    }

    pub fn graphql(
        schema: Schema<QueryRoot, MutationRoot, EmptySubscription>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("graphql")
            .and(async_graphql_warp::graphql(schema).and_then(super::handlers::graphql))
    }
}

mod handlers {
    use super::MutationRoot;
    use super::QueryRoot;
    use async_graphql::http::GQLResponse;
    use async_graphql::{EmptySubscription, QueryBuilder, Schema};
    use warp::Reply;

    pub async fn graphql(
        (schema, builder): (
            Schema<QueryRoot, MutationRoot, EmptySubscription>,
            QueryBuilder,
        ),
    ) -> Result<impl warp::Reply, warp::reject::Rejection> {
        let resp = builder.execute(&schema).await;
        Ok(warp::reply::json(&GQLResponse(resp)).into_response())
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    println!("Steps#1");

    EnvironmentProvider::default()
        .credentials()
        .await
        .ok()
        .expect("NO ENVIRONMENT VARIABLE PROVIDED!");

    let dynamodb_client = DynamoDbClient::new(Region::EuCentral1);

    let app_state = AppState {
        db_client: dynamodb_client,
    };

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .register_type::<graphql_schema::Node>()
        .data(app_state.clone())
        .finish()
        .clone();

    let graphql = filters::graphql(schema);

    let index = warp::path::end().map(|| "Ok");

    let routes = index.or(filters::health()).or(graphql);

    println!("Server is runnning on PORT 8080");

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
