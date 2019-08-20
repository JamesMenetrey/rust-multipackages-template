pub enum Animal {
    Horse,
    Rabbit
}

mod conversions;

pub use conversions::human_to_animal_age;