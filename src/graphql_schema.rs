use crate::coffee_order_type::GQLCoffeeOrder;
use crate::coffee_type::CoffeeType;
use crate::database::{
    fetch_coffee_orders, find_coffee_order_by_id, get_customer_name_from_pk, save_coffee_order,
    SaveCoffeeOrderInput,
};
use crate::AppState;
use async_graphql::{Context, FieldError, FieldResult, InputObject, Interface, Object, ID};
use rusoto_dynamodb::DynamoDbClient;
use serde_json::json;

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
        let node_definition = match extract_node_definition_from_relay_node_id(&id) {
            Some(node) => node,
            None => return Ok(None),
        };
        let db = ctx.data::<AppState>().db_client.clone();

        match node_definition.node_type.as_str() {
            "CoffeeOrder" => {
                println!("Query DynamoDB for CoffeeOrder by id: {}", id);
                let entity_id = node_definition.node_id;

                let result = match find_coffee_order_by_id(db, entity_id).await {
                    Ok(order) => match order {
                        Some(o) => o,
                        None => {
                            return Ok(None);
                        }
                    },
                    Err(_) => {
                        return Ok(None);
                    }
                };

                Ok(Some(
                    GQLCoffeeOrder {
                        id: result.id,
                        coffee_type: result.coffee_type,
                        customer_name: result.customer_name,
                    }
                    .into(),
                ))
            }
            _ => Ok(None),
        }
    }

    async fn coffee_orders(&self, ctx: &Context<'_>) -> FieldResult<Option<Vec<GQLCoffeeOrder>>> {
        let db = ctx.data::<AppState>().db_client.clone();
        let output = match fetch_coffee_orders(db).await {
            Ok(output) => output,
            Err(_) => return Ok(None),
        };

        let mut coffee_orders = vec![];

        let vec_orders = output.items.unwrap();

        for i in 0..vec_orders.len() {
            let order = vec_orders[i].clone();
            if let (Some(pk), Some(coffee_type)) = (
                order.get("pk").unwrap().s.clone(),
                order.get("coffeeType").unwrap().s.clone(),
            ) {
                let coffee_type_enum: CoffeeType = coffee_type.parse().unwrap();

                let order = GQLCoffeeOrder {
                    id: as_relay_id("CoffeeType", pk.clone()),
                    coffee_type: coffee_type_enum,
                    customer_name: get_customer_name_from_pk(pk),
                };
                coffee_orders.push(order);
            }
        }

        Ok(Some(coffee_orders))
    }
}

#[InputObject]
pub struct OrderCoffeeInput {
    #[field(name = "coffeType")]
    coffee_type: CoffeeType,
    #[field(name = "customerName")]
    customer_name: String,
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    #[field(name = "orderCoffee")]
    async fn order_coffee(
        &self,
        ctx: &Context<'_>,
        input: OrderCoffeeInput,
    ) -> FieldResult<GQLCoffeeOrder> {
        let db_client: DynamoDbClient = ctx.data::<AppState>().db_client.clone();

        let mutation_input = SaveCoffeeOrderInput {
            coffee_type: input.coffee_type,
            customer_name: input.customer_name,
        };

        let coffee_order = match save_coffee_order(db_client, mutation_input).await {
            Ok(coffee_order) => coffee_order,
            Err(err) => {
                let my_extension =
                    json!({ "details": "Could not find a coffee order", "error": err.to_string() });
                return Err(FieldError(
                    String::from("Cannot Save Coffee Order"),
                    Some(my_extension),
                ));
            }
        };

        Ok(coffee_order)
    }
}

#[derive(Debug)]
pub struct NodeDefinition {
    node_type: String,
    node_id: String,
}

pub fn extract_node_definition_from_relay_node_id(relay_node_id: &str) -> Option<NodeDefinition> {
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

pub fn as_relay_id(entity_name: &str, id: String) -> ID {
    let relay_id = base64::encode(format!("{}:{}", entity_name, id)).into();
    relay_id
}
