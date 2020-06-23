use rusoto_core::Region;
use rusoto_credential::{EnvironmentProvider, ProvideAwsCredentials, CredentialsError};
use rusoto_dynamodb::DynamoDbClient;

#[derive(Clone)]
pub struct AppState {
    pub db_client: DynamoDbClient,
}

pub async fn get_app_state() -> Result<AppState, CredentialsError> {
    match EnvironmentProvider::default()
        .credentials()
        .await {
            Ok(ok) => ok,
            Err(e) => {
                return Err(e)
            }
        };

    let dynamodb_client = DynamoDbClient::new(Region::EuCentral1);

    Ok(AppState {
        db_client: dynamodb_client,
    })
}
