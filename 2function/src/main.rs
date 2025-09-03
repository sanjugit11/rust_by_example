fn main() {
    greet();
    name_print("sanjeev");
    check_odd(4);
    println!("{}", check_odd_return(5));
}
fn greet (){
     println!("Hello, world!");
 }
fn name_print(user:&str) {
    println!("Hello ,{user}");
}

fn check_odd(num : u32) {
   let result:bool = num % 2 == 0 ;
   println!("given ,{num} is odd number ,{result}");
}
fn check_odd_return(num : u32) -> bool {
   num % 2 == 0    // no semicolon use here 
}