pub fn main() {
    // different_arrays();
    animals();
}
// in rust arrays arrays are fixed, if I want to increment and add on to an array it needs to the growing array aka the vector
pub fn different_arrays() {

let mut weather: Vec<String> = Vec::new();

for i in 0..3 {
    weather.push(format!("Weather {}", i));
}

println!("new text {:?}", weather);


}


// I want to have one variable that contains 5 literal strings of animal types then a second variable that has names of animals, and in the for loop i take the 
// first variable and insert them inside the second variable and im not borrowing data im just taking the data and owning it and discarding the first variable

pub fn animals () {
    // firt variable owns the data ( super simple vector of animal types )
    let animal_types: Vec<String> = vec![
        "dog".to_string(),
        "cat".to_string(),
        "horse".to_string(),
        "lion".to_string(),
        "eagle".to_string(),
    ];

    // second variable: will own the moved data
    let mut animal_names: Vec<String> = Vec::new();

    // Move ownership from animal_types into animal_names 
    // The reason why I would use this type of logic is becasuse if I was reading an api call and got json object that data from a third party needs to saved into my OWNED variable for my codebase logic to have memory of that data, borrowing will only force 
    // data be alive
    for animal in animal_types {
        animal_names.push(animal);
    }

    // animal types is now invalid ( ownership moved )
    // println!("{:?}", animal_types);
    println!("{:?}", animal_names);
}