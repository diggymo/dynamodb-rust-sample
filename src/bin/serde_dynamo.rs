use std::collections::{HashMap, HashSet};

use ddb_test::util::{create_dynamodb_client, User};
use serde_dynamo::aws_sdk_dynamodb_1::{from_item, to_item};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let client = create_dynamodb_client().await;

    let user_a = User {
        id: Uuid::new_v4().to_string(),
        name: "John".to_string(),
        age: 20,
        is_married: false,
        friends: vec![Uuid::new_v4().to_string(), Uuid::new_v4().to_string()],
        metadata: HashMap::from([(
            "favorite_songs".to_string(),
            vec!["song1".to_string(), "song2".to_string()],
        )]),
        sikaku: HashSet::from(["AWS SAP".to_string(), "DB Specialist".to_string()]),
        pet_name: None,
    };

    client
        .put_item()
        .table_name("users")
        .set_item(Some(to_item(user_a).unwrap()))
        .send()
        .await
        .unwrap();

    let results = client.scan().table_name("users").send().await.unwrap();
    for user_ddb_item in results.items.unwrap() {
        let new_user_a: User = from_item(user_ddb_item).unwrap();
        dbg!(new_user_a);
    }
}
