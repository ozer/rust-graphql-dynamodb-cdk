use crate::coffee_order_type::GQLCoffeeOrder;
use crate::coffee_type::CoffeeType;
use crate::graphql_schema::as_relay_id;

use chrono::Utc;
use rusoto_core::RusotoError;
use rusoto_dynamodb::{
    AttributeValue, DynamoDb, DynamoDbClient, PutItemError, PutItemInput, QueryError, QueryInput,
    ScanError, ScanInput, ScanOutput,
};
use std::collections::HashMap;
use uuid::Uuid;

pub fn string_attribute_value(s: String) -> AttributeValue {
    AttributeValue {
        b: None,
        bool: None,
        bs: None,
        l: None,
        m: None,
        s: Some(s.to_string()),
        ns: None,
        null: None,
        n: None,
        ss: None,
    }
}

pub fn number_attribute_value(s: String) -> AttributeValue {
    AttributeValue {
        b: None,
        bool: None,
        bs: None,
        l: None,
        m: None,
        s: None,
        ns: None,
        null: None,
        n: Some(s),
        ss: None,
    }
}

pub struct SaveCoffeeOrderInput {
    pub coffee_type: CoffeeType,
    pub customer_name: String,
}

pub async fn fetch_coffee_orders(db: DynamoDbClient) -> Result<ScanOutput, RusotoError<ScanError>> {
    let scan_input = ScanInput {
        table_name: String::from("CoffeeShop"),
        limit: Some(50),
        attributes_to_get: None,
        conditional_operator: None,
        consistent_read: None,
        exclusive_start_key: None,
        expression_attribute_names: None,
        expression_attribute_values: None,
        filter_expression: None,
        index_name: None,
        projection_expression: None,
        return_consumed_capacity: None,
        scan_filter: None,
        segment: None,
        total_segments: None,
        select: None,
    };

    match db.scan(scan_input).await {
        Ok(output) => Ok(output),
        Err(error) => {
            println!("Error: {:?}", error);
            Err(error)
        }
    }
}

pub async fn save_coffee_order(
    db: DynamoDbClient,
    input: SaveCoffeeOrderInput,
) -> Result<GQLCoffeeOrder, RusotoError<PutItemError>> {
    let mut order = HashMap::new();

    let order_id: String = Uuid::new_v4().to_string();

    input.customer_name.to_string().push_str(&order_id);

    let composite_pk = format!("{}#{}", input.customer_name.to_string(), order_id);

    let now = Utc::now().timestamp_millis().to_string();

    let pk = string_attribute_value(composite_pk);
    let sk = number_attribute_value(now);

    let coffee_type = string_attribute_value(input.coffee_type.to_string());

    order.insert("pk".to_string(), pk);
    order.insert("sk".to_string(), sk);
    order.insert("coffeeType".to_string(), coffee_type);

    let put_item_input = PutItemInput {
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

    match db.put_item(put_item_input).await {
        Ok(_) => Ok(GQLCoffeeOrder {
            id: as_relay_id("CoffeeOrder".to_string().as_ref(), order_id),
            customer_name: input.customer_name,
            coffee_type: input.coffee_type,
        }),
        Err(error) => {
            println!("Error: {:?}", error);
            Err(error)
        }
    }
}

pub async fn find_coffee_order_by_id(
    db: DynamoDbClient,
    id: String,
) -> Result<Option<GQLCoffeeOrder>, RusotoError<QueryError>> {
    let key_condition_expression = "#pk = :order_id".to_string();

    let mut expression_attribute_names = HashMap::new();
    expression_attribute_names.insert("#pk".to_string(), "pk".to_string());

    let mut expression_attribute_values = HashMap::new();
    let pk_expr_attr_value = string_attribute_value(id.to_string());
    expression_attribute_values.insert(":order_id".to_string(), pk_expr_attr_value);

    let query_input = QueryInput {
        attributes_to_get: None,
        conditional_operator: None,
        consistent_read: Some(true),
        exclusive_start_key: None,
        projection_expression: None,
        query_filter: None,
        expression_attribute_names: Some(expression_attribute_names),
        expression_attribute_values: Some(expression_attribute_values),
        filter_expression: None,
        key_conditions: None,
        return_consumed_capacity: None,
        scan_index_forward: Some(true),
        table_name: String::from("CoffeeShop"),
        index_name: None,
        key_condition_expression: Some(key_condition_expression),
        select: None,
        limit: Some(1),
    };

    let order = match db.query(query_input).await {
        Ok(output) => {
            println!("Success!");
            Ok(output)
        }
        Err(error) => {
            println!("Error at finding coffee order by id:[{}] -> {:?}", id, error);
            return Err(error);
        }
    };

    let result = match order {
        Ok(aa) => aa,
        Err(e) => return Err(e),
    };

    let result_count = result.clone().count;

    let aa: Option<i64> = Some(1 as i64);

    if result_count < aa {
        return Ok(None);
    }

    let items = match result.clone().items {
        Some(i) => i,
        None => return Ok(None),
    };

    let first_item = match items.first() {
        Some(first) => first,
        None => return Ok(None),
    };

    let pk_attr = match first_item.get(&"pk".to_string()) {
        Some(attr) => attr.clone(),
        None => return Ok(None),
    };

    let pk = match pk_attr.clone().s {
        Some(s) => s,
        None => return Ok(None),
    };

    // let sk_attr = match first_item.get(&"sk".to_string()) {
    //     Some(attr) => attr.clone(),
    //     None => number_attribute_value("".to_string()),
    // };

    // let sk = match sk_attr.clone().n {
    //     Some(n) => n,
    //     None => String::from(""),
    // };

    let coffee_type_attr = match first_item.get(&"coffeeType".to_string()) {
        Some(attr) => match attr.clone().s {
            Some(coffee_type) => coffee_type,
            None => String::from(""),
        },
        None => String::from(""),
    };

    let coffee_type_enum: CoffeeType = coffee_type_attr.parse().unwrap();

    let customer_name = get_customer_name_from_pk(pk.clone());
    let gql_order = Some(GQLCoffeeOrder {
        id: as_relay_id("CoffeeOrder".to_string().as_ref(), pk),
        customer_name,
        coffee_type: coffee_type_enum,
    });

    Ok(gql_order)
}

pub fn get_customer_name_from_pk(pk: String) -> String {
    let splitted: Vec<&str> = pk.split("#").collect();
    match splitted.first() {
        Some(&n) => n.to_string(),
        None => "".to_string(),
    }
}
