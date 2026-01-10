pub fn main() {
    test();
}

pub fn test() {
    // &String shouldn't be used for the most part - &str is much better and more flexible whereas &Strinng is not flexible and the usecases are very rare it seems like
    // let book_slices: &[&String; 3] = &[
    //     &"Harry Potter".to_string(),
    //     &"wizard of ox".to_string(),
    //     &"takumi".to_string(),
    // ];
    // println!("book_slices : {:?}", book_slices);

    // Mutabiliity:
    let mut stone_cold: String = String::from("Hell");
    //  {:?} this is meant for developers to see the interal structure
    println!("stone cold as {:?}", stone_cold);

    for stone in 0..3 {
        stone_cold.push_str("new animal, ");
    }
    println!("stone cold: {:?}", stone_cold);

    // testing out the internals
    let testArray: &[&str] = &["human", "angel", "devil"];
    println!("testing the structure {:?}", testArray);
    // the {} only becomes a problem if there are multiple strings in the array because rust does not want to guess how it should display for the user
    println!("non dev struct {}", testArray.join(", "));

    // however in this case it works because theres only string literal and rust knows excactly how to show it to the user
    let singleString: &str = "name";
    println!("string lit {}", singleString);
}



// let singleString : String = "blaze";
// println!("name: {}", singleString);

// // output: blaze

// let multipleStrings : Vec<String> = vec![
//     "harry",
//     "andrew",
//     "emma",
//     "silas",
// ]

// print!("names {}", multipleStrings.join(", "));
// // rust bc it doesn't know how to seperate and it doesn't want to guess
// // output: harry, andrew, emma, silas,

// print!("names {:?}", multipleStrings.join(", "));
// // ["harry", "andrew", "emma", "silas"]