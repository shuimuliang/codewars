// You can create 3 coffee recipes:
//
// Black = Black coffee
// Cubano = Cubano coffee + Brown sugar
// Americano = Americano coffee + Milk with 0.5% fat
// And you can add a lot of extra sugar and extra milk in any coffee, e.g.:
//
// Black + Milk with 3.2% fat + Brown sugar
// Cubano + Brown sugar + Brown sugar + Regular sugar
// Americano + Milk with 3.2% fat + Milk with 0% fat + Regular sugar

#[derive(Debug)]
pub struct Coffee {
    pub sort: String,
    pub milk: Vec<Milk>,
    pub sugar: Vec<Sugar>,
}

#[derive(Debug)]
pub struct Milk {
    pub fat: f32,
}

#[derive(Debug)]
pub struct Sugar {
    sort: String,
}


struct CoffeeBuilder {
    sort: String,
    milk: Vec<Milk>,
    sugar: Vec<Sugar>,
}

// const CoffeeBlack: &'static str = "Black";
// const CoffeeCubano: &'static str = "Cubano";
// const CoffeeAmericano: &'static str = "Americano";
// const SugarBrown: &'static str = "Brown";
// const SugarRegular: &'static str = "Regular";
// const Fat5: f32 = 0.5;
// const Fat32: f32 = 3.2;

impl CoffeeBuilder {
    #[allow(dead_code)]
    fn new() -> CoffeeBuilder {
        CoffeeBuilder {
            sort: String::new(),
            milk: Vec::<Milk>::new(),
            sugar: Vec::<Sugar>::new(),
        }
    }

    #[allow(dead_code)]
    fn set_black_coffee(mut self) -> CoffeeBuilder {
        // Black = Black coffee
        self.sort = String::from("Black");
        self
    }

    #[allow(dead_code)]
    fn set_cubano_coffee(mut self) -> CoffeeBuilder {
        // Cubano = Cubano coffee + Brown sugar
        self.sort = String::from("Cubano");
        self.sugar.push(Sugar { sort: String::from("Brown") });
        self
    }

    #[allow(dead_code)]
    fn set_antoccino_coffee(mut self) -> CoffeeBuilder {
        // Americano = Americano coffee + Milk with 0.5% fat
        self.sort = String::from("Americano");
        self.milk.push(Milk { fat: 0.5 });
        self
    }

    #[allow(dead_code)]
    fn with_milk(mut self, fat: f32) -> CoffeeBuilder {
        self.milk.push(Milk { fat });
        self
    }

    #[allow(dead_code)]
    fn with_sugar(mut self, sort: String) -> CoffeeBuilder {
        self.sugar.push(Sugar { sort });
        self
    }

    #[allow(dead_code)]
    fn build(self) -> Coffee {
        // Coffee {
        //     sort: self.sort.clone(),
        //     milk: self.milk.into_iter().collect::<Vec<Milk>>(),
        //     sugar: self.sugar.into_iter().collect::<Vec<Sugar>>(),
        // }
        Coffee {
            sort: self.sort,
            milk: self.milk,
            sugar: self.sugar,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        let coffee = CoffeeBuilder::new()
            .set_black_coffee()
            .with_sugar("Regular".to_string())
            .with_milk(3.2)
            .build();
        assert_eq!(format!("{:?}", coffee), "Coffee { sort: \"Black\", milk: [Milk { fat: 3.2 }], sugar: [Sugar { sort: \"Regular\" }] }");

        let coffee = CoffeeBuilder::new()
            .set_antoccino_coffee()
            .build();
        assert_eq!(format!("{:?}", coffee), "Coffee { sort: \"Americano\", milk: [Milk { fat: 0.5 }], sugar: [] }");

        let coffee = CoffeeBuilder::new()
            .set_cubano_coffee()
            .build();
        assert_eq!(format!("{:?}", coffee), "Coffee { sort: \"Cubano\", milk: [], sugar: [Sugar { sort: \"Brown\" }] }");
    }
}