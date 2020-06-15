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

impl std::str::FromStr for CoffeeType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Cappucino" => Ok(CoffeeType::Cappuccino),
            "Americano" => Ok(CoffeeType::Americano),
            "Espress" => Ok(CoffeeType::Espresso),
            "Macchiato" => Ok(CoffeeType::Macchiato),
            "Mocha" => Ok(CoffeeType::Mocha),
            "Latte" => Ok(CoffeeType::Latte),
            _ => Err(format!("'{}' is not a valid value for CoffeeOrder!", s))
        }
    }
}