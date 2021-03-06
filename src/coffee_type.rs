use async_graphql::Enum;

#[Enum]
#[derive(Debug)]
pub enum CoffeeType {
    Cappuccino,
    Americano,
    Espresso,
    Macchiato,
    Mocha,
    Latte,
}

impl std::fmt::Display for CoffeeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
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
            _ => Err(format!("'{}' is not a valid value for CoffeeOrder!", s)),
        }
    }
}
