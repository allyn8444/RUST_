// =============== CHARS ================== //

// -character, single letter

// -wrapped by single quotes

// ---

// - PROBLEM 1
    

    
//     // Make it work
//     use std::mem::size_of_val;
//     fn main() {
//         let c1 = 'a';
//         assert_eq!(size_of_val(&c1),1); 
    
//         let c2 = '中';
//         assert_eq!(size_of_val(&c2),3); 
    
//         println!("Success!");
//     } 
 
//     SOLUTION:
    

    
//     use std::mem::size_of_val;
//     fn main() {
//         let c1: char = 'a';
//         assert_eq!(size_of_val(&c1),4); // change to 4
//         // println!("{}", size_of_val(&c1)); // 4 bytes
    
//         let c2: char = '中';
//         assert_eq!(size_of_val(&c2),4); // change to 4
//         // println!("{}", size_of_val(&c2)); // 4 bytes
    
//         println!("Success!");
//     } 

// - PROBLEM 2
    

    
//     // Make it work
//     fn main() {
//         let c1 = "中";
//         print_char(c1);
//     } 
    
//     fn print_char(c : char) {
//         println!("{}", c);
//      
//     SOLUTION:
    

//     fn main() {
//         let c1 : char= '中'; // change double quotes to single quotes
//         print_char(c1);
//     } 
    
//     fn print_char(c : char) {
//         println!("{}", c);
//     }
    
//     // double quotes are for string
//  ================ BOOL ================ //
// true or false

// ---

// - PROBLEM 1
    

    
//     // Make println! work
//     fn main() {
//         let _f: bool = false;
    
//         let t = true;
//         if !t {
//             println!("Success!");
//         }
//     } 
//     SOLUTION
    

//     fn main() {
//         let _f: bool = false;
    
//         let t = false; // change true to false
//         if !t {
//             println!("Success!");
//         }
//     } 
 

// - PROBLEM 2
    

    
//     // Make it work
//     fn main() {
//         let f = true;
//         let t = true && false;
//         assert_eq!(t, f);
    
//         println!("Success!");
//      
//     SOLUTION:
    

//     fn main() {
//         let f = true;
//         let t = true && true; // change false to true
//         assert_eq!(t, f);
    
//         println!("Success!");
//  `


// ========== UNIT TYPE ============ //

// The unit type is represented by (), and it signifies the absence of a meaningful value.
// It is similar to void in languages like C and C++, but it is an actual type in Rust.
// The unit type has exactly one value, also written as (), which is known as the unit value.


// - PROBLEM 1
    
//     // Make it work, don't modify `implicitly_ret_unit` !
//     fn main() {
//         let _v: () = ();
    
//         let v = (2, 3);
//         assert_eq!(v, implicitly_ret_unit());
    
//         println!("Success!");
//     }
    
//     fn implicitly_ret_unit() {
//         println!("I will return a ()");
//     }
    
//     // Don't use this one
//     fn explicitly_ret_unit() -> () {
//         println!("I will return a ()");
//     }
    
//     SOLUTION:
//     #[allow(unused_variables)]
//     // Make it work, don't modify `implicitly_ret_unit` !
//     fn main() {
//         let _v: () = ();
    
//         let v: (i32, i32) = (2, 3);
//         assert_eq!(_v, implicitly_ret_unit()); // change v to _v (since _v has the unit type value)
    
//         println!("Success!");
//     }
    
//     fn implicitly_ret_unit() {
//         println!("I will return a ()");
//     }
    
//     // Don't use this one
//     // fn explicitly_ret_unit() -> () {
//     //     println!("I will return a ()");
//     // }
    

// - PROBLEM 2
    

    
//     // Modify `4` in assert to make it work
//     use std::mem::size_of_val;
//     fn main() {
//         let unit: () = ();
//         assert!(size_of_val(&unit) == 4);
    
//         println!("Success!");
//     }
    
//     SOLUTION:
    

//     use std::mem::size_of_val;
//     fn main() {
//         let unit: () = ();
//         // println!("{}", size_of_val(&unit)); // 0 byte
//         assert!(size_of_val(&unit) == 0); // change 4 to zero
    
//         println!("Success!");
//     }