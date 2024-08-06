
// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
// fn main() {
//     let x: i32 = 5;
//     {
//         let x = 12;
//         assert_eq!(x, 5);
//     }

//     assert_eq!(x, 12);

//     let x = 42;
//     println!("{}", x); // Prints "42".
// }



fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12); // switch to 12 instead of 5 (scope)
    }

    assert_eq!(x, 5); // switch to 5 instead of 12

    let x = 42;
    println!("{}", x); // Prints "42".
}

// Output: 42