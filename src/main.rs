extern crate log;

mod app_state;
mod coffee_order_type;
mod coffee_type;
mod database;
mod graphql_schema;

use async_graphql::{EmptySubscription, Schema};
use dotenv;
use warp::Filter;

use app_state::{get_app_state, AppState};
use graphql_schema::{MutationRoot, QueryRoot};

pub mod filters {
    use super::app_state::get_app_state;
    use super::MutationRoot;
    use super::QueryRoot;
    use async_graphql::{EmptySubscription, Schema};
    use warp::http::status::StatusCode;
    use warp::Filter;

    pub fn health() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path("health").and_then(|| async move {
            match get_app_state().await {
                Ok(ok) => Ok::<StatusCode, std::convert::Infallible>(StatusCode::OK),
                Err(e) => {
                    println!("Error at get_app_state: {}", e);
                    return Ok::<StatusCode, std::convert::Infallible>(
                        StatusCode::INTERNAL_SERVER_ERROR,
                    );
                }
            }
        })
    }

    pub fn graphql(
        schema: Schema<QueryRoot, MutationRoot, EmptySubscription>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let graphql_log = warp::log::custom(|info| {
            eprintln!(
                "{} {} {} {:?} {:?}",
                info.method(),
                info.path(),
                info.status(),
                info.user_agent(),
                info.elapsed(),
            );
        });
        warp::body::content_length_limit(1024 * 32).and(
            warp::path("graphql")
                .and(async_graphql_warp::graphql(schema).and_then(super::handlers::graphql))
                .with(graphql_log),
        )
    }
}

pub mod handlers {
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
    pretty_env_logger::init();

    let app_state = get_app_state().await;

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .register_type::<graphql_schema::Node>()
        .data(app_state.clone())
        .finish()
        .clone();

    let graphql = filters::graphql(schema);

    let index = warp::path::end().map(|| "Ok");

    let routes = index.or(filters::health()).or(graphql);

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}

// #[cfg(test)]
// mod tests {
//     use super::filters;
//     use warp::http::StatusCode;
//     use warp::test::request;

//     #[tokio::test]
//     async fn test_health() {
//         dotenv::dotenv().ok();

//         let health = filters::health();

//         let resp = request().method("GET").path("/health").reply(&health).await;

//         assert_eq!(resp.status(), StatusCode::OK);
//     }
// }
