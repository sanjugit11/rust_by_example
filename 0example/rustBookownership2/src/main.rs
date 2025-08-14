use std::io;
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len();
}

// we pass &s1 into calculate_length and, in its definition, we take &String
//  rather than String. These ampersands represent references, and they allow
//   you to refer to some value without taking ownership of it
// he opposite of referencing by using & is dereferencing, which is accomplished 
// with the dereference operator, *
// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

/////////////////////////////////////////////////
// fn main (){
//     let x =  String::from("this is ownership");
//     let y = x.clone();
//     println!("this is output {} {}",x, y);
// }

// fn main(){
//     let x = 5;
//     let y = 6.clone();
//     println!("value {} {}",x, y);
// }

/////////////////////
// fn main() {
//     let s = String::from("hello");  // s comes into scope
//     takes_ownership(s);             // s's value moves into the function...
//                                     // ... and so is no longer valid here

//     let x = 5;                      // x comes into scope
//     makes_copy(x);                  // x would move into the function,
//                                     // but i32 is Copy, so it's okay to still
//                                     // use x afterward
// } // Here, x goes out of scope, then s. But because s's value was moved, nothing// special happens.

// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{}", some_integer);
// }


///////////////////////////////////////////////////
// fn main(){
// let s1 = String::from("hello");

// let (s2, len) = calculate_length(s1);

// println!("The length of '{}' is {}.", s2, len);
// }

// fn calculate_length(s: String) -> (String, usize) {
// let length = s.len(); // len() returns the length of a String

// (s, length)
// }

////////////////////////////////////////////////////////////////