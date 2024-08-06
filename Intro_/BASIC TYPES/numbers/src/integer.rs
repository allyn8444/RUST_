// - PROBLEM 1
    
    
//     // Remove something to make it work
//     fn main() {
//         let x: i32 = 5;
//         let mut y: u32 = 5;
    
//         y = x;
        
//         let z = 10; // Type of z ? 
    
//         println!("Success!");
//     }
    
//     SOLUTION:
    
//     #[allow(unused_variables)]
    
//     // Remove something to make it work
//     fn main() {
//         let x = 5; // remove explicit assigned data type
//         let mut y = 5;
    
//         y = x;
        
//         let z = 10; // Type of z ? 
    
//         println!("Success!");
//     }
    

// - PROBLEM 2
    
    
//     // Fill the blank
//     fn main() {
//         let v: u16 = 38_u8 as __;
    
//         println!("Success!");
//     }
    
//     SOLUTION:
    
    
//     // Fill the blank
//     fn main() {
//         let v: u16 = 38_u8 as u16; // "as" keyword and then retype u16
    
//         println!("Success!");
//     }
    
//     Explanation:
    
//     variable `v` is explicitly set to `u16` data type
    
//     but the value `38` is assigned as `u8` data type.
    
//     So we used “as” keyword with `u16` to make sure that variable `v` can store it
    

// - PROBLEM 3
    
    
//     // Modify `assert_eq!` to make it work
//     fn main() {
//         let x = 5;
//         assert_eq!("u32".to_string(), type_of(&x));
    
//         println!("Success!");
//     }
    
//     // Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
//     fn type_of<T>(_: &T) -> String {
//         format!("{}", std::any::type_name::<T>())
//     }
    
    
//     SOLUTION:
    
//     fn main() {
//         let x: u32 = 5; // i32 by default, change to u32
//         assert_eq!("u32".to_string(), type_of(&x)); 
//         // check if u32 is equal to type of x (which is u32 also)
    
//         println!("Success!");
//     }
    
//     // Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
//     fn type_of<T>(_: &T) -> String {
//         format!("{}", std::any::type_name::<T>()) // returns "u32"
//     }
    

// - PROBLEM 4
    
    
//     // Fill the blanks to make it work
//     fn main() {
//         assert_eq!(i8::MAX, __); 
//         assert_eq!(u8::MAX, __); 
    
//         println!("Success!");
//     }
    
//     SOLUTION:
    
    
//     // Fill the blanks to make it work
//     fn main() {
//         assert_eq!(i8::MAX, 127); // enter max value in i8 data type 
//         assert_eq!(u8::MAX, 255); // enter max value in u8 data type
    
//         println!("Success!");
//     }
    
//     Explanation:
    
//     -Every data type has its own maximum value
    

// - PROBLEM 5
    
    
//     // Fix errors and panics to make it work
//     fn main() {
//        let v1 = 251_u8 + 8;
//        let v2 = i8::checked_add(251, 8).unwrap();
//        println!("{},{}",v1,v2);
//     }
    
//     Error: Values are overflowing
    
//     SOLUTION 1:
    
//     fn main() {
//        let v1 = 247_u8 + 8; // 255 (MAX of u8) - 8 = 247 
//        let v2 = i8::checked_add(119, 8).unwrap(); // 127 (MAX of i8) - 8 = 119
//        println!("{},{}",v1,v2);
//     }
    
//     SOLUTION 2:
    
//     fn main() {
//        let v1 = 251_u16 + 8; // change u8 to u16
//        let v2 = i16::checked_add(251, 8).unwrap(); // change i8 to i16
//        println!("{},{}",v1,v2);
//     }
    

// - PROBLEM 6
    
    
//     // Modify `assert!` to make it work
//     fn main() {
//         let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
//         assert!(v == 1579);
    
//         println!("Success!");
//     }
    
//     SOLUTION:
    
//     fn main() {
//         let v = 1_024 + 0xff + 0o77 + 0b1111_1111; // 1024 + 255 + 63 + 255 = 1597
//         assert!(v == 1597); // change  1579 to 1597
    
//         println!("Success!");
//     }
    
//     Explanation:
    
//     `1_024` = 1024. Decimal (underscore is just the delimeter)
    
//     `0xff` = 255. Hexidecimal
    
//     `0o77` = 63. Octal
    
//     `0b1111_1111` = 255. Binary