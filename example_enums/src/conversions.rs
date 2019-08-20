//! The aging functions have been extracted from the charts of age-converter.com.

use example_maths::multiply;
use crate::Animal;

fn human_to_horse_age(human_age: u32) -> u32 {
    multiply(human_age, 3)
}

fn human_to_rabbit_age(human_age: u32) -> u32 {
    match human_age {
        0 => 0,
        _ => 20 + multiply(human_age - 1, 8)
    }
}

pub fn human_to_animal_age(animal: &Animal, human_age: u32) -> u32 {
    match animal {
        Animal::Horse => human_to_horse_age(human_age),
        Animal::Rabbit => human_to_rabbit_age(human_age)
    }
}