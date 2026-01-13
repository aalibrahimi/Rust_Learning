pub fn main() {
    scopes()
}
pub fn scopes() {
    // This is a literal scope
    let x: u32 = 10;
    {
        // this is an isolated scope ( valid in rust apparently )
        let y = x + 10;
        println!("y should be: {}", y);
        println!("x is: {}", x);
    }

    println!("x actually is: {}", x);
    println!("y actually is: {}", y);
}

/*
big scope ( variable within ( x )  ) {
    // bc the small is within the big, that must mean that the big should and has access inside of the scope, otherwise why even have the small in the first place
     {
        scope with in  and varialbes within ( y )
      } ->

     //  even though the big guy can look inside, the value within the small guy cannot be seen outside of the scope

     }

*/
