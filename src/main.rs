// Stack and the heap

// Stack: fast, automatic, memory for values with a known size and lifetime
// Heap: dynamic memory for values whose size or lifetime is not known at compile time

// ============== STACK =======================
// Think of the stack as:
/*
    a vertical stack of boxes
    - Each function call gets its own box
    - when the function ends, the entire box is destroyed instantly

    --properties--
    - very fast
    - fixed size known at compile time
    - memory is automatically freed
    - no fragmentation
    - cannot grow or shrink dynamically

``` rust                                    |   Memory
        fn main() {                         |   | y = 20 |
        let x = 10;                         |   | x = 10 | <- top of stack
        let y = 20;                         |
        }                                   | ( for some reason the stack starts from the bottom going up, bottom goes first ) *when main ends, both are gone instantly* no clean up code runs.
```


// ============== The Heap =====================
The heap is:
    - a large pool of memory
    - you ask for memory at runtime
    - you get back a pointer to that memory
    - you must decided when to free it

    --Properties--
    - slower than stack
    - size decided at runtime
    - must be explicity managed
    - data lives independently of function stack frames
*/

fn main() {

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number array: {:?}", numbers); 

    // let mix = [1, 2, "apple", true];
    // println!("mix: {:?}", mix);
    let fruits: [&str; 3] = ["bananan", "apple", "orange"];
    // println!("first fruits {:?}", fruits[0]);
    // println!("second fruits {:?} ", fruits[1]);
    // println!("third fruits {:?}", fruits[2]);
    // println!("all fruits: {:?} ", fruits);

    // now on to tuples:
    let human: (String, i32, bool) = ( String::from("Alice")  , 30, false);  // or Alice.to_string()
    print!("human: {:?} ", human);

    // array inside of tuple 4 different elements
    // A slice is (pointer, length)'
    /*
    the slice itself is stored on the stack.
        so stack memory looks like
            // let animals = ["lion", "tiger", "hippo"];
            animals -> ( ptr -> array, len = 3 )
            array   -> [ ptr, ptr, ptr ]
         -  No heap involved anywhere in this example.
     */


    /*
        My question regardin: let animals: &[&str] = &["lion", "tiger", "hippo"];
        why cant we just reference with this syntax &[str] or [&str] instead of &[&str] ?

        Answer:  
            - firstly str is an unsized type and cannot exist by itself in the memory
            - secondly you cannot replace &[&str] with &[str] because a slice must point to element that have a known layhout and str does not.

            step 1:
                # what str actually is:
                    - str is NOT a normal type
                    - str is a dynamiclaly sized type
                    - its size is not known at compile time
                    - it cannot be stored directly on the stack
                    - it cannot be an element of an array

                    str has no size by itself
                    
        */
    let insanity = ("Hanif", 21, true, [1, 2, 3, 4, 5]);
    println!("let the insanity show me : {:?}", insanity);

    // slices: [1, 2, 3, 4, 5]
    // my initial though process before: I thought animals had to have existed first before being able to reference it with a new or second variable type point towards that data *so that it doesn't have to clone or duplicate data in the memory for no reason"
    // however thats not the case at all, reference simply means borrowing an array memory space, and if I want the variable to OWN or Borrow data, apparently rust does not assume ownership, so as the developer I have to choose.
    let animals = ["lion", "tiger", "hippo"];
    println!("animals: {:?} ", animals)

}
