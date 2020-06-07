use crate::coffee_type::CoffeeType;

use rusoto_core::RusotoError;
use rusoto_dynamodb::{ DynamoDb, AttributeValue, PutItemError, DynamoDbClient, PutItemInput, PutItemOutput};
use std::collections::HashMap;

pub struct SaveCoffeeOrderInput {
    pub coffee_type: CoffeeType,
    pub customer_name: String,
}

pub async fn save_coffee_order(
    db: DynamoDbClient,
    input: SaveCoffeeOrderInput,
) -> Result<PutItemOutput, RusotoError<PutItemError>> {

    let mut order = HashMap::new();

    let pk = AttributeValue {
        b: None,
        bool: None,
        bs: None,
        l: None,
        m: None,
        s: Some(input.coffee_type.to_string()),
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

    match db.put_item(input).await {
        Ok(output) => {
            println!("output: {:?}", output);
            println!("Success!");
            Ok(output)
        }
        Err(error) => {
            println!("Error: {:?}", error);
            Err(error)
        }
    }
}
