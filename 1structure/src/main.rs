// #[derive(Debug)]  //important for  printing whole struct
// struct Person{
//     name :String,
//     age : u32,
//     city :String,
// }
// fn main() {
//     let data = Person{
//         name: "John".to_string(),
//         age: 30,
//         city: "New York".to_string(),
//     };
//     println!("{:?}",data);   //whole data is printed

//     let data2 =  Person{
//         name: "second".to_string(),
//         age:23,
//         city:"london".to_string(),
//     };
//     println!("{} {} {}",data2.name,data2.age,data2.city);
// }
///////////////////////////////////////////////////////
// struct Example(u8,u8,u8);
// fn main(){
//     let a = Example(1,2,3);
//     println!("{:?}",a);
// }

/////////////////input struct/////////////////
// #[derive(Debug)]  //important for  printing whole struct // but not in input fields//
use std::io;
struct Example{
    name: String,
    age: u32,
}
fn main(){
    let mut data = Example{
        name: String::new(),
        age: 0
    };
    let mut name_input = String::new();
    println!("enter the name");
    io::stdin().read_line(&mut name_input).expect("Error reading name_input");
    data.name = name_input;

    let mut age_input = String::new();
    println!("enter the age");
    io::stdin().read_line(& mut age_input).expect("Error reading age_input");
    data.age = age_input.trim().parse().expect("Error input age");
    println!("name==>{} age==>{}",data.name,data.age);   //print in newline
    // println!("{:?}",data);  //not work here
    
    // let mut class_input = string::new();
    // io::stdin().read_line(&mut class_input).expect()
}

