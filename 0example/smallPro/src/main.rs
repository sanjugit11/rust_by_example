///////////////////-------project 1
// use::std::io;
// fn main() {
//     let mut input = String::new();
//     println!("please enter the table input");
//     io::stdin().read_line(&mut input).expect("Error reading");
//     let  input_converted :u32 = input.trim().parse().expect("Error parsing");   //input string value in to number
//     println!("input-- {}",input_converted);

//         let mut cal = 1;
//         ////with while
//         // while cal <= 10{
//         //     println!("=====>{}",cal * input_converted);
//         //     cal += 1;
//         // }
//         ////with for
//         for i in 1..=10{
//             println!("=====>{}",i * input_converted);
//         }
// }

///////////////////------project 2

use std::io;
fn main (){
    let mut input = String::new();
    println!("enter the amount greater then 10");
    io::stdin().read_line(&mut input).expect("error input");
    let input_converted :u32 = input.trim().parse().expect("Error parsing"); 
    if input_converted > 10{
        for i in 1..input_converted{
        let mut value:u32 = i % 2 ;
        if value!= 0{
            println!("odd number===>{}",i);
        }else{
            println!("even number===>{}",i);
        }

        }
    }
}