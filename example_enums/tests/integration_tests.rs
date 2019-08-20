//! Testing can also be done in a separate project

use example_enums::{human_to_animal_age, Animal};

#[test]
fn test_rabbit_age() {
    assert_eq!(human_to_animal_age(&Animal::Rabbit, 5), 52);
}