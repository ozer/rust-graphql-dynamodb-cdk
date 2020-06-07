use async_graphql::Enum;

#[Enum]
#[derive(Debug)]
pub enum CoffeeType {
    Cappuccino = "Cappuccino",
    Americano = "Americano",
    Espresso = "Espresso",
    Macchiato = "Macchiato",
    Mocha = "Mocha",
    Latte = "Latte",
}

impl std::fmt::Display for CoffeeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
