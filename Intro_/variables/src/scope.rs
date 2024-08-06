fn main() {
    let x: i32 = 10;
    let y: i32 = 5; // define Y in the top, outside, to become a global scoped variable 
    
    {
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y); 
}

// Output:
// The value of x is 10 and value of y is 5 
// The value of x is 10 and value of y is 5



// ANOTHER EXAMPLE:



// fn main() {
//    define_x(); // calling a function
// }

// fn define_x() {
//     let x: &str = "hello";

//      println!("{}, world", x); 
// }