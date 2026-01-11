use std::io::{self, stdin};


fn main() {
    // understanding_slices();
    // partTwoRustVideo();
    let height = extract_height();
    how_tall_are_you(height);
}

pub fn understanding_slices() {
    let sliceUP: &[&str] = &["lion", "giraffe", "penguin"];
    println!("the animals are: {:?}", sliceUP);
}

pub fn partTwoRustVideo () {
    // hes currently talking about mutable
    // String Slice
    let word : String = String::from("learning new rust");
    let slice: &str = &word[0..5]; // we do not need to duplicate data, just create a reference variable pointing towards our initial String
    println!("word value: {}", slice);
}


pub fn read_heightg_from_user() -> f64 {
    let mut input = String::new();

    println!("Enter your heightg in cm:");

    io::stdin()
        .read_line(&mut input)
        .expect("There was an error reading your response");

    input
        .trim() // remove new line
        .parse::<f64>() //convert to number
        .expect("Please enter a valid number")
}

pub fn how_tall_are_you(height: f64) {
     println!("So you are {} cm", height);
}

// The above method is perfect for grabbing just a float response from the user, but here I want to sift throught the sentence the user might responde with and extract just the number
// eg. "How tall are you "          user response: "I am 160 cm tall",   what currently happens : Please enter a valid number: ParseFloatError { kind: Invalid }
// so this function will parse the float from the rest of the sentence.
pub fn extract_height() -> f64 {

    let mut extractedHeight = String::new();
    println!("How tall are you in cm");

    stdin()
        .read_line(&mut extractedHeight)
        .expect("There was an error parsing your stuff out");

    // here is the for loop for parsing out the whitespace and word for word
    for word in extractedHeight.split_whitespace()  {
        // the ok()  check and extracts in one step and cannot panic ( think of it as : only run this block if parsing succeeded )
        if let Ok(number) = word.parse::<f64>() { // word.parse::<64> Examines the string and attempts to convert it into a number and then decides if it succeeds or fails
            return number;
        }
    }
    panic!("no number found in put");


}