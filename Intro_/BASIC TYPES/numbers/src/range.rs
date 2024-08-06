// // A `Range` represents a sequence of values that can be iterated over. Ranges are commonly used in loops and other iterative constructs. They are defined using two values: a start and an end. The range syntax in Rust is concise and expressive.


// - PROBLEM 1    
//     Modify `assert!` to make it work 
    
//     Make `println!` output: 97 - 122
//     fn main() {
//         let mut sum = 0;
//         for i in -3..2 {
//             sum += i
//         }
    
//         assert!(sum == -3);
    
//         for c in 'a'..='z' {
//             println!("{}",c);
//         }
//     }
    
    
//     SOLUTION:
//     fn main() {
//         let mut sum = 0;
//         for i in -3..2 { // 2 is excluded (no equal sign)
//             sum += i // will end at -5
//         }
    
//         assert!(sum == -3); // change -3 to -5
    
//         for c in 'a'..='z' { // z is included because it has equal sign
//             println!("{}",c as u8); // change value of "c" into ASCII
//         }
//     }
    

// - PROBLEM 2
//     // Fill the blanks
//     use std::ops::{Range, RangeInclusive};
//     fn main() {
//         assert_eq!((1..__), Range{ start: 1, end: 5 });
//         assert_eq!((1..__), RangeInclusive::new(1, 5));
    
//         println!("Success!");
//     }

    
//     SOLUTION:
//     use std::ops::{Range, RangeInclusive};
//     fn main() {
//         assert_eq!((1..5), Range{ start: 1, end: 5 }); // no equal sign because it is exclusive/excluded
//         assert_eq!((1..=5), RangeInclusive::new(1, 5)); // need EQUAL sign since it is inclusive
    
//         println!("Success!");
//     }