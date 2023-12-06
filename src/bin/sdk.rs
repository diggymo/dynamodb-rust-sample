use std::collections::{HashMap, HashSet};

use ddb_test::util::{create_dynamodb_client, User};
use uuid::Uuid;

use aws_sdk_dynamodb::types::AttributeValue::{Bool, Null, Ss, L, M, N, S};

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

    // 1.登録処理
    client
        .put_item()
        .table_name("users")
        .set_item(Some(HashMap::from([
            ("id".to_string(), S(user_a.id)),
            ("name".to_string(), S(user_a.name)),
            ("age".to_string(), N(user_a.age.to_string())),
            ("is_married".to_string(), Bool(user_a.is_married)),
            (
                "friends".to_string(),
                L(user_a.friends.into_iter().map(|friend| S(friend)).collect()),
            ),
            (
                "metadata".to_string(),
                M(user_a
                    .metadata
                    .into_iter()
                    .map(|(key, value)| (key, Ss(value)))
                    .collect()),
            ),
            (
                "sikaku".to_string(),
                Ss(user_a.sikaku.into_iter().collect()),
            ),
            (
                "pet_name".to_string(),
                user_a
                    .pet_name
                    .map_or_else(|| Null(true), |pet_name| S(pet_name)),
            ),
        ])))
        .send()
        .await
        .unwrap();

    let results = client.scan().table_name("users").send().await.unwrap();

    for user_ddb_item in results.items.unwrap() {
        // 2.取得処理
        let new_user_a = User {
            id: user_ddb_item.get("id").unwrap().as_s().unwrap().to_string(),
            name: user_ddb_item
                .get("name")
                .unwrap()
                .as_s()
                .unwrap()
                .to_string(),
            age: user_ddb_item
                .get("age")
                .unwrap()
                .as_n()
                .unwrap()
                .parse::<u8>()
                .unwrap(),
            is_married: user_ddb_item
                .get("is_married")
                .unwrap()
                .as_bool()
                .unwrap()
                .to_owned(),
            friends: user_ddb_item
                .get("friends")
                .unwrap()
                .as_l()
                .unwrap()
                .into_iter()
                .map(|attribute_value| attribute_value.as_s().unwrap().to_string())
                .collect(),
            sikaku: user_ddb_item
                .get("sikaku")
                .unwrap()
                .as_ss()
                .unwrap()
                .to_vec()
                .into_iter()
                .collect(),
            metadata: user_ddb_item
                .get("metadata")
                .unwrap()
                .as_m()
                .unwrap()
                .into_iter()
                .map(|(key, value)| (key.to_string(), value.as_ss().unwrap().to_vec()))
                .collect(),
            pet_name: user_ddb_item
                .get("pet_name")
                .unwrap()
                .as_s()
                .map_or(None, |pet_name| Some(pet_name.to_string())),
        };

        dbg!(new_user_a);
    }
}
