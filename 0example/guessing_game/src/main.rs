
//// reverse loop ////
// fn main() {
//     for number in (1..4).rev() {
//         println!("{number}!");
//     }
//     println!("LIFTOFF!!!");
// }

//// for loop ////
// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     for element in a {
//         println!("the value is: {element}");
//     }
// }

//// loop 
// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;
//         println!("The counter is :{counter}");

//         if counter == 10 {
//             break counter * 2;
//         }
//     };
//     println!("The result is :{result}");
// }

/////////Both random number and input number////////////
// use std::io;
// use rand::Rng;
// fn main(){
//     println!("this is my first programm to input number");
//     println!("this is random number:{}",rand::thread_rng().gen_range(1..100));
//     println!("Enter the amount");

//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("error while input the number");

//     println!("your gussed:{}",input);
// }

/////////////random number compare//////////////
// use rand::Rng;
// use std::cmp::Ordering;  //comapre the value
// use std::io;

// fn main() {
//     // --snip--
//     let mut guess = String::new();

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

//     let guess: u32 = guess.trim().parse().expect("Please type a number!");
//     println!("Your guessed: {guess}");

//     let random :u32   = rand::thread_rng().gen_range(1..10);
//     println!("Your random: {random}");

//     match guess.cmp(& random) {
//         Ordering::Less => println!("Too small!"),
//         Ordering::Greater => println!("Too big!"),
//         Ordering::Equal => {
//             println!("You win!");
//             return;
//         }
//     }
// }

///////////////guess////////////////////
// use std::io;

// fn main() {
//     println!("Guess the number!");

//     println!("Please input your guess.");

//     let mut guess = String::new();

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

//     println!("You guessed: {guess}");
// }

// ----------------------------------------------
// use std::io;
// use rand::Rng;

// fn main() {
//     println!("Guess the number!");

//     let secret_number = rand::thread_rng().gen_range(1..=100);

//     println!("The secret number is: {secret_number}");

//     println!("Please input your guess.");

//     let mut guess = String::new();

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

//     println!("You guessed: {guess}");
// }