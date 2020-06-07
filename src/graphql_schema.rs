use crate::coffee_type::CoffeeType;
use crate::database::{save_coffee_order, SaveCoffeeOrderInput};
use crate::AppState;
use async_graphql::{Context, FieldError, FieldResult, InputObject, Interface, Object, ID};
use rusoto_dynamodb::{AttributeValue, Condition, DynamoDb, DynamoDbClient, ScanInput};
use serde_json::json;
use std::collections::HashMap;

pub struct GQLCoffeeOrder {
    id: ID,
    coffee_type: CoffeeType,
}

#[Object(name = "CoffeeOrder")]
impl GQLCoffeeOrder {
    pub async fn id(&self) -> ID {
        self.id.clone()
    }

    #[field(name = "coffeeType")]
    pub async fn coffee_type(&self) -> CoffeeType {
        self.coffee_type
    }
}

#[Interface(field(name = "id", type = "ID"), arg(name = "id", type = "String"))]
pub enum Node {
    CoffeeOrder(GQLCoffeeOrder),
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn me(&self) -> FieldResult<String> {
        Ok(String::from("Ozericco!"))
    }

    async fn node(&self, ctx: &Context<'_>, id: String) -> FieldResult<Option<Node>> {
        let node_definition = match extract_id_from_relay_node_id(&id) {
            Some(node) => node,
            None => return Ok(None),
        };
        let db = ctx.data::<AppState>().db_client.clone();

        match node_definition.node_type.as_str() {
            "CoffeeOrder" => {
                println!("Query DynamoDB for CoffeeOrder by id: {}", id);

                let mut scan_input = HashMap::new();

                let attr = AttributeValue {
                    b: None,
                    bool: None,
                    bs: None,
                    l: None,
                    m: None,
                    s: Some(node_definition.node_id),
                    ns: None,
                    null: None,
                    n: None,
                    ss: None,
                };

                let mut attribute_value_list: Vec<AttributeValue> = Vec::new();

                attribute_value_list.insert(0, attr);

                let condition = Condition {
                    attribute_value_list: Some(attribute_value_list),
                    comparison_operator: String::from("EQ"),
                };

                scan_input.insert(String::from("pk"), condition);

                let scan_input: ScanInput = ScanInput {
                    attributes_to_get: None,
                    conditional_operator: None,
                    consistent_read: None,
                    exclusive_start_key: None,
                    expression_attribute_names: None,
                    expression_attribute_values: None,
                    filter_expression: None,
                    index_name: None,
                    limit: None,
                    projection_expression: None,
                    return_consumed_capacity: None,
                    segment: None,
                    select: None,
                    scan_filter: Some(scan_input),
                    total_segments: None,
                    table_name: String::from("CoffeeShop"),
                };

                let order = match db.scan(scan_input).await {
                    Ok(order) => {
                        println!("Order {:?}", order);
                        Some(order)
                    }
                    Err(e) => {
                        println!("Error at scan for coffee order {:?}", e);
                        None
                    }
                };

                Ok(Some(
                    GQLCoffeeOrder {
                        id: String::from("coffeeOrderId").into(),
                        coffee_type: CoffeeType::Cappuccino,
                    }
                    .into(),
                ))
            }
            _ => Ok(None),
        }
    }
}

#[InputObject]
pub struct OrderCoffeeInput {
    #[field(name = "coffeType")]
    coffee_type: CoffeeType,
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
        println!("hey!!");
        let db_client: DynamoDbClient = ctx.data::<AppState>().db_client.clone();

        println!("hey22!!");

        let mutation_input = SaveCoffeeOrderInput {
            coffee_type: input.coffee_type,
            customer_name: String::from("Ozer"),
        };

        let result = match save_coffee_order(db_client, mutation_input).await {
            Ok(coffee_order) => println!("result: {:?}", coffee_order),
            Err(err) => {
                println!("Cant save coffee order!");

                let my_extension =
                    json!({ "details": "Could not find a room guest", "error": err.to_string() });
                return Err(FieldError(
                    String::from("Cannot Save Coffee Order"),
                    Some(my_extension),
                ));
            }
        };

        Ok("hey".to_string())
    }
}

#[derive(Debug)]
pub struct NodeDefinition {
    node_type: String,
    node_id: String,
}

pub fn extract_id_from_relay_node_id(relay_node_id: &str) -> Option<NodeDefinition> {
    let decoded = match base64::decode(relay_node_id) {
        Ok(result) => result,
        Err(e) => {
            println!("Error at decode node_id{}", e);
            Vec::new()
        }
    };

    let splitted = match String::from_utf8(decoded) {
        Ok(result) => result,
        Err(e) => {
            println!("Error at split node_id: {}", e);
            String::from("")
        }
    };

    let node_definition: Vec<&str> = splitted.split(":").collect();

    let node_type = match node_definition.first() {
        Some(result) => result,
        None => "",
    };

    let node_id = match node_definition.last() {
        Some(result) => result,
        None => "",
    };

    Some(NodeDefinition {
        node_id: String::from(node_id),
        node_type: String::from(node_type),
    })
}

fn as_relay_id(entity_name: &str, id: i32) -> ID {
    base64::encode(format!("{}:{}", entity_name, id)).into()
}
