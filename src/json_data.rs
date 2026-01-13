// strongly typed rust model
//  - apparently this is where rust shines

use serde::Deserialize;
use std::error::Error;
use std::fs::File;

// whats happening:
/*
   Deserialize tells rust: this struct can be built from json
   Field names must match json keyhs
   types must be compatible
*/
#[derive(Debug, Deserialize)]
struct Person {
    name: String,
    age: u32,
    height_cm: f64,
}
// Result<(), Box<dyn Error>>   and what it means?
/*
   it means this function can succeed or fail.
   ()
   - this () is the success value and it means "nothing meaningful to return - just sucess".
   - so Ok(()) means everything worked

   Box<dyn Error>
   this is the error value
   - it means "if something fails, return anyu kind of error"
   Error -> a standard error trait
   dyn -> dynamic ( many possible error types )
   Box -> put it on the heap so they all fit the same type


*/

//  ============= 1. WHEN IS BOX NEEDED? ==============
/*
    The Problem:
      -  Different error types have different sizes in memory. AND Rust must know the size of a value at compile time.

      These are different types with different sizes.
        File::open(...)        → io::Error
        serde_json::from_reader(...) → serde_json::Error

    The Solution:
      - Box<T> stores the value on the heap and keeps only a fixed-size pointer on the stack.
        Box<dyn Error>

     - “Put whatever error this is on the heap, and just point to it.” My question now. how big is the heap? how much memory is allocated there? and as long as I point my data to the heap, I can just run functions without fully being explicity about the size types?
        in one sentence "Box erases size differences by storing the value behind a pointer."


================ 2. What dyn Actually Means ================
    dyn = dynamic dispatch
    Error is a trait, not a concrete type    // describes behavior
    - interesting it seems like rust has to know beforehand what error youre returning first, dyn error basically allows multiple errors types to be treated as one ( very interesting ). My question is why can't we do this more often or in every use cases?


Final Mental Note:
    Box      → handles unknown size
    dyn      → allows many types
    ?        → returns errors automatically


*/

pub fn main() -> Result<(), Box<dyn Error>> {
    // The question mark means "Try to open this file. if it fails, stop immediately and return the error."
    let file = File::open("src/data.json")?;

    /*  If I didn't have the shorthand ? I would have to write this:
            let file = match File::open("data.json") {
            Ok(f) => f,
            Err(e) => return Err(Box::new(e)),
    };

     */
    let people: Vec<Person> = serde_json::from_reader(file)?;

    // next task is to filter out tall people
    let oldPerson = people.iter().filter(|p| p.age > 25);
    for person in oldPerson {
        print!(
            "{} is {} years old and {} cm tall\n ",
            person.name, person.age, person.height_cm
        );
    }

    Ok(())
}

/*
    Stack:  pointer (known size)
    Heap:   error value (unknown size at compile time)


    How much memory does Box<dyn Error> use?

        Two parts:

            On the stack
           - A fixed-size pointer (usually 8 bytes on 64-bit systems)

            On the heap
           - The actual error object (size depends on the error type)

*/
