
// fn main() {
//     let x = 1; 
// }

// Warning: unused variable: `x`


// SOLUTION 1

// fn main() {
//     let _x = 1; // using underscore before the variable name
// }

// No warnings

// SOLUTION 2
#[allow(unused_variables)]

fn main() {
    let x = 1; 
}