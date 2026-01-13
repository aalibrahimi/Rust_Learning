use std::{io, ops::Add};
                                 
pub fn main() {

  let bmi = calculate_bmi();
    println!("Your BMI is {:.2}", bmi);
}

pub fn human_id(age: u32, name: &str, height: f64) {
    println!(
        "{} is {} years old and their height is {}",
        name, age, height
    );

    let _Value= {
        let price = 5;
        let qty = 10;

       let harms = price * qty;
       // im so confused, why can't I just return price * qty here? it returns to me an empty paranethesis if I do
       // value returns only after I gave it a veriable and return that variable
       harms
    };
    println!("The sum value of price vs qauntity is now {:?} ", _Value);

// ================== too idiomatic ( trying to use add *incorrect* ) ============================ //
    let x = 5;
    let y = 10;
    let z = x + y;

    // let mut summary = summary.add(z); 
    let summary = |x:i32| { x.add(z) };
    println!("the summary is: {:?}", summary(0));


//================== This is how you would implement the add() method =========================== //
    let money = 200;
    let dollars = 156;
    println!("i have this {} in my bank", money.add(dollars) );


//================== How to use closurfes outsided of methods  =============================== // 

let add_to_z = |n: i32| n + z;
println!("the summary is: {}", add_to_z(0)); // the zero is there for the sole purpose of triggering the function

}

// Creating a Bmi Function
// BMI = height(kg)/height(m)^2
fn calculate_bmi() -> f64  {
    // I did an example of shadow
    // let mut weight = String::new();
    let mut weight_input = String::new();
    println!("What is your current weight?");

    io::stdin()
        .read_line(&mut weight_input)
        .expect("could not see your message well");
    let weight = weight_input
        .trim()
        .parse::<f64>()
        .expect("just enter number only");


    // height input
    let mut height_input = String::new();
    println!("what is your height now? ");

    io::stdin()
        .read_line(&mut height_input)
        .expect("I said numbers only");

    let mut height = height_input
        .trim()
        .parse::<f64>()
        .expect("i said numbers only lolzy");    

    let height_meters = height / 100.0;

     weight / height_meters.powf(2.0) 

}

// Expressions
// -------------
// true & false
// add(3, 5)
// if condition { value1 } else { value2 }
// ({ code })

