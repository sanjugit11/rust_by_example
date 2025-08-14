///// referance and dereference
// fn main() {
//     let x = 5;
//     assert_eq!( x , 5);
//     println!("first==> {}", x);

//     let y = &x; //set y to a reference to x
//     // assert_eq!( y , 5);     //makes error
//     assert_eq!(*y , 5);     //good
//    println!("first==> {}",*y);
//      // dereference y
// }
/////////////////////////------------------------
// fn main() {
//     let data = 
    // [201, 227, 187, 104, 139, 28, 241, 96, 87, 105, 202, 238, 196, 91, 208,
    // 21, 56, 127, 176, 200, 126, 23, 48, 173, 219, 10, 119, 179, 21, 91, 158,
    // 9, 242, 129, 12, 235, 99, 216, 141, 202, 25, 87, 27, 116, 105, 84, 46,
    // 200, 4, 39, 142, 24, 74, 73, 185, 150, 228, 164, 240, 201, 53, 147, 151,
    // 98];

//     // Get the length of the array
//     let length = data.len();

//     println!("Length of the array: {}", length);
// }
///////////////wroking////////////////------------------------------
extern crate bs58;
use std::convert::TryInto;
fn main() {
    // Given secret key bytes
    let secret_key_bytes: [u8; 64] = 
    [201, 227, 187, 104, 139, 28, 241, 96, 87, 105, 202, 238, 196, 91, 208,
    21, 56, 127, 176, 200, 126, 23, 48, 173, 219, 10, 119, 179, 21, 91, 158,
    9, 242, 129, 12, 235, 99, 216, 141, 202, 25, 87, 27, 116, 105, 84, 46,
    200, 4, 39, 142, 24, 74, 73, 185, 150, 228, 164, 240, 201, 53, 147, 151,
    98];

    // Convert secret key bytes to a base58-encoded private key
    let private_key = bs58::encode(&secret_key_bytes).into_string();

    println!("Derived Private Key: {}", private_key);
}
//////////////////////////////-------------------------------

//////////////add ///////////////
// fn main() {
//     add(10, 213);
//     // println!("Hello, world!",c);
// }

// fn add(a:u32,b:u32  ){
//  let c :u32 = a+ b;
//  println!("Hello, world!{}",c);
// } 
/////////////////2/////////////////////////////////////
///////////////smart pointer/////////

// fn main (){
//     let a =2;
//     let result = stack_only(a);
//     dbg!(result);
// }
//    fn stack_only(b:i32) -> i32{
//     let c = 3;
//     return b + c + stack_and_heap();
//    }

//    fn stack_and_heap() ->i32 {
//     let d = 33 ;
//     let e = Box::new(8);  //smart pointer allocation at 8
//     println!("here is the e {}:",e);
//     return d + *e;
//    }

//////////////////3/////////////////////////////
// extern crate rand;
// use rand::Rng;
// fn main(){
//     let rando = rand::random::<u8>();
//     println!("random{}",rando);
// }

// RUST code: Generate Random Numbers within a Range
// use rand::Rng;
  
// fn main() {
//     let mut rng = rand::thread_rng();
//     let num: u8 = rng.gen_range(0..10);
//     println!("Random number between 0 and 9: {}", num);
    // let rando = rand::random::<u8>();
    // println!("random{}",rando);
    // for _x in 0..5{
    // let num: u8 = rng.gen_range(0..10);
    // println!("Random number between 0 and 9: {}", num);
    // }
// }