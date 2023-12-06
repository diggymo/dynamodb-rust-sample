use std::collections::{HashMap, HashSet};

use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};

use aws_sdk_dynamodb::Client as DynamoDbClient;
use serde::{Deserialize, Serialize};

pub async fn create_dynamodb_client() -> DynamoDbClient {
    let region_provider: RegionProviderChain =
        RegionProviderChain::first_try("ap-northeast-3").or_default_provider();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    let client = DynamoDbClient::new(&config);
    client
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub age: u8,
    pub is_married: bool,
    pub friends: Vec<String>,
    pub metadata: HashMap<String, Vec<String>>,
    pub sikaku: HashSet<String>,
    pub pet_name: Option<String>,
}
