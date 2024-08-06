
// fn main() {
//     let (x, y);
//     (x,..) = (3, 4);
//     [.., y] = [1, 2];
  
//     assert_eq!([x,y], __);   // Fill the blank to make the code work

//     println!("Success!");
// } 



fn main() {
    let (x, y); // declaring 2 variables at once

    (x,..) = (3, 4); // double dots means that "don't store the value in that part"
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3,2]);

    println!("Success!");
} 

// Output: Success!