// use std::io;
fn main(){
    // let mut _new = String::new();
    // io::stdin().read_line(&mut _new).expect("Error reading");
    // println!("input-- {}",_new);
    // let buf = 8u64.pow(10) ;
    // println!("{}", buf);

    // if buf <= 100{
    //     println!("less than 100");
    // }else{
    //     println!("more than 100");
    // }

  /////------- loop
  let mut a = 0;
    loop{
        println!("loop");
        a +=1;
        println!("a==>{}",a);
        if a == 10{
            break;
        }
    }
///////-------while
    let mut b = 0;
    while b < 10{
        let mut odd = b % 2;
        if odd != 0{
            println!("odd number {}",b);
        }
        b += 1;
        if b==10{
            break;
        }
    }

////////------for
    // for i in (0..10){    //range
    // for i in (0..10).rev(){    //in reverse order
    for i in (0..10).step_by(3) {    // skip the third element
        // println!("all number{}",i);
        let mut c = i % 2;
        if c != 0 {
            println!("odd number===>{}",i);
        }
    }

//////-------string
    // use std :: io;
    // let mut input = String::new();
    // println!("input the string here: \n");
    // io::stdin().read_line(&mut input).expect("input error");
    // let test = "test"; //
    // let s = String::from(input);
    // let s1 = String::from("hello world 1");
    // let s2 = String::from("          nhello world 2  ");
    // println!("==> {}",test);
    // println!("first name {:?}",s);    // :? will add new line 
    // println!("second name {}",s1);
    // let s2 = s2.trim_start().to_string();    // trim the spaces before
    // println!("third string ====>{}",s2);

    // //concatinate the value
    // let mut concat = s + &s1 ;
    // println!(" concat value==>{:?}",concat); // concat the value
    // println!(" this is value==> {}",&s2[3..9]);  //slicing the value    
} 