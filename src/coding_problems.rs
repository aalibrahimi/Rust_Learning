// This is where I'll  have Ai Generate me problems and I have to come up with logic to solve in rust so I could get comfortable with the syntax
/*
    Requirements

    1. Create a variable that owns exactly 5 animal type strings
        - Example categories: mammal, bird, reptile, fish, insect
        - Use owned strings (String), not &str

    2. Create a second variable that starts empty
    - This will be your final storage

    3. Write a loop that:
    - Takes ownership of each animal type
    - Inserts it into the second variable
    - The first variable must be unusable afterward

    4. fter the loop:
    -  Print the second variable
    - Demonstrate (mentally or via comment) that the first variable cannot be used
*/
pub fn main() {
    categorizing_animals();
}

// Problem: Categorizing Animals ====== Goal : Practice ownership transfer, vectors, and iteration.

 pub fn categorizing_animals() {
    // creating a vector full of strings that are own with categories of animals eaching string being owned
    let category: Vec<String>  = vec![
        "mammal".to_string(),
        "bird".to_string(),
        "reptile".to_string(),
        "fish".to_string(),
        "insect".to_string(),
    ];

    // create a variable that starts empty but needs to be able to intake all the animal types so I assume it has to be a vector ( changing array )
    let mut inserting_category: Vec<String> = Vec::new();

    for animal in category {
        inserting_category.push(format!("{} class", animal));
    }
    // the category variable has now become useless and cannot be printed out! data moved! 
    println!("Moved ownership to new vector variable {:?}", inserting_category);

 } // Succesfully completed code problem 1