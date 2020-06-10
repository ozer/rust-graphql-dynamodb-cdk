use async_graphql::{Context, FieldError, FieldResult, InputObject, Interface, Object, ID};
use crate::coffee_type::CoffeeType;

pub struct GQLCoffeeOrder {
  pub id: ID,
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
}
