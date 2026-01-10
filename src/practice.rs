fn main() {
    understanding_slices();
}

pub fn understanding_slices() {
    let sliceUP: &[&str] = &["lion", "giraffe", "penguin"];
    println!("the animals are: {:?}", sliceUP);
}
