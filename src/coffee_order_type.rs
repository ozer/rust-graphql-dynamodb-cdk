use crate::coffee_type::CoffeeType;
use async_graphql::{Object, ID};

#[derive(Debug)]
pub struct GQLCoffeeOrder {
    pub id: ID,
    pub customer_name: String,
    pub coffee_type: CoffeeType,
}

#[Object(name = "CoffeeOrder")]
impl GQLCoffeeOrder {
    pub async fn id(&self) -> ID {
        self.id.clone()
    }

    #[field(name = "coffeeType")]
    pub async fn coffee_type(&self) -> CoffeeType {
        self.coffee_type.into()
    }

    #[field(name = "customerName")]
    pub async fn customer_name(&self) -> String {
        self.customer_name.clone()
    }
}
