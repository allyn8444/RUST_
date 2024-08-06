// - PROBLEM 1:
//     // Fill the blank to make it work
//     fn main() {
//         let x = 1_000.000_1; // ?
//         let y: f32 = 0.12; // f32
//         let z = 0.01_f64; // f64
    
//         assert_eq!(type_of(&x), "__".to_string());
//         println!("Success!");
//     }
    
//     fn type_of<T>(_: &T) -> String {
//         format!("{}", std::any::type_name::<T>())
//     }

    
//     SOLUTION:
//     // Fill the blank to make it work
//     fn main() {
//         let x = 1_000.000_1; // 1000.0001 (this is an f64)
//         let y: f32 = 0.12; // f32
//         let z = 0.01_f64; // f64
    
//         assert_eq!(type_of(&x), "f64".to_string()); // type of x (f64) is equal to f64
//         println!("Success!");
//     }
    
//     fn type_of<T>(_: &T) -> String {
//         format!("{}", std::any::type_name::<T>())
//     }

    

// - PROBLEM 2:
//     // Make it work in 2 distinct ways
//     fn main() {
//         assert!(0.1+0.2==0.3);
    
//         println!("Success!");
//     }

    
//     SOLUTION 1:
    
//     // assert_eq! and _f32
//     fn main() {
//         assert_eq!(0.1_f32 + 0.2_f32, 0.3_f32);
    
//         println!("Success!");
//     }
    
//     Explanation:
    
//     0.1 + 0.2 doesn’t equate to 0.3 but rather something like 0.300000002
    
//     -So to limit the values, we added _f32
    
//     -since by default, floats have f64 data type

    
//     SOLUTION 2:
//     // using "as" keyword
//     fn main() {
//         assert_eq!(0.1 as f32 + 0.2 as f32, 0.3 as f32);
    
//         println!("Success!");
//     }
    
//     -just the same function as SOLUTION 1. just different way