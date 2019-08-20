use std::io;
use example_enums::{Animal, human_to_animal_age};
use std::io::Write;

fn main() {
    // Define the arrays of animals
    let animals_mapping = [Animal::Horse, Animal::Rabbit];
    let labels_mapping = ["Horse", "Rabbit"];
    debug_assert_eq!(animals_mapping.len(), labels_mapping.len(), "The two arrays must store corresponding values.");

    // Print out the animals available
    println!("Select the animal you would like to convert your age in:");
    for i in 0..animals_mapping.len() {
        println!(" {}. {}", i, &labels_mapping[i]);
    }

    // Request the animal the user would like to evaluate
    print!("Number of the animal: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let animal_index = input.trim().parse::<usize>().expect("Unable to parse the number of the animal");
    let animal = animals_mapping.get(animal_index).expect("The number does not exist");

    // Request the age of the user
    print!("Enter your age: ");
    io::stdout().flush().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();

    let human_age = input.trim().parse::<u32>().expect("Invalid age");

    // Print the result, using the other crates
    println!();
    println!("Your age ({}) as a {} is {}.", human_age, labels_mapping[animal_index], human_to_animal_age(animal, human_age));
}
