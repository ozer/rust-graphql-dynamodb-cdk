use rusoto_core::Region;
use rusoto_credential::{EnvironmentProvider, ProvideAwsCredentials};
use rusoto_dynamodb::DynamoDbClient;

#[derive(Clone)]
pub struct AppState {
    pub db_client: DynamoDbClient,
}

pub async fn get_app_state() -> AppState {
    EnvironmentProvider::default()
        .credentials()
        .await
        .ok()
        .expect("NO ENVIRONMENT VARIABLE PROVIDED!");

    let dynamodb_client = DynamoDbClient::new(Region::EuCentral1);

    AppState {
        db_client: dynamodb_client,
    }
}
