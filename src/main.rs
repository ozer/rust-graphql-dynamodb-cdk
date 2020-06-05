use async_graphql::{Context, EmptySubscription, FieldResult, InputObject, Object, Schema};
use dotenv;
use rusoto_core::Region;
use rusoto_credential::{EnvironmentProvider, ProvideAwsCredentials};
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, PutItemInput};
use std::collections::HashMap;
use warp::Filter;

pub struct QueryRoot;

#[derive(Clone)]
pub struct AppState {
    pub db_client: DynamoDbClient,
}

#[Object]
impl QueryRoot {
    async fn me(&self) -> FieldResult<String> {
        Ok(String::from("Ozericco!"))
    }
}

#[InputObject]
pub struct OrderCoffeeInput {
    #[field(name = "coffeeName")]
    coffee_name: String,
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    #[field(name = "orderCoffee")]
    async fn order_coffee(
        &self,
        ctx: &Context<'_>,
        input: OrderCoffeeInput,
    ) -> FieldResult<String> {
        let db_client: DynamoDbClient = ctx.data::<AppState>().db_client.clone();

        let mut order = HashMap::new();

        let pk = AttributeValue {
            b: None,
            bool: None,
            bs: None,
            l: None,
            m: None,
            s: Some(input.coffee_name),
            ns: None,
            null: None,
            n: None,
            ss: None,
        };

        let sk = AttributeValue {
            b: None,
            bool: None,
            bs: None,
            l: None,
            m: None,
            s: Some("ozer".to_string()),
            ns: None,
            null: None,
            n: None,
            ss: None,
        };

        order.insert("pk".to_string(), pk);
        order.insert("sk".to_string(), sk);

        let input = PutItemInput {
            condition_expression: None,
            conditional_operator: None,
            expected: None,
            expression_attribute_names: None,
            table_name: String::from("CoffeeShop"),
            item: order,
            return_consumed_capacity: None,
            return_item_collection_metrics: None,
            expression_attribute_values: None,
            return_values: None,
        };

        match db_client.put_item(input).await {
            Ok(output) => {
                println!("output: {:?}", output);
                println!("Success!");
            }
            Err(error) => {
                println!("Error: {:?}", error);
            }
        }

        Ok("hey".to_string())
    }
}

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

    EnvironmentProvider::default()
        .credentials()
        .await
        .ok()
        .expect("NO ENVIRONMENT VARIABLE PROVIDED!");

    let dynamodb_client = DynamoDbClient::new(Region::EuCentral1);

    println!("Steps#1");

    let app_state = AppState {
        db_client: dynamodb_client,
    };

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(app_state.clone())
        .finish()
        .clone();

    println!("Steps#2");

    let graphql = filters::graphql(schema);

    println!("Steps#3");

    let index = warp::path::end().map(|| "Ok");

    let routes = index.or(filters::health()).or(graphql);

    println!("Steps#4");

    println!("Server is runnning on PORT 8080");

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
