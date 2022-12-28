// Copyright 2023 Oxide Computer Company

use typify::import_types;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MyFruit {
    seeds: (),
}

import_types!(
    schema = "../example.json",
    patch = {
        Veggie = {
            rename = "Vegetable",
        },
    },
    replace = {
        Fruit = MyFruit: ?Display,
    }
);

#[test]
fn test_main() {
    match main() {
        Ok(it) => it,
        Err(_) => (),
    };
}

fn main() -> Result<(), String> {
    let veg = Vegetable {
        veggie_name: String::from("carrots"),
        veggie_like: true,
    };
    let veggies = Veggies {
        fruits: vec![String::from("apple"), String::from("mango")],
        vegetables: vec![veg],
        id_num: Some(VeggiesIdNum::try_from(25)?),
    };
    println!("{:?}", veggies);
    let fov = FruitOrVeg::Fruit(MyFruit { seeds: () });
    println!("{:?}", fov);
    Ok(())
}
