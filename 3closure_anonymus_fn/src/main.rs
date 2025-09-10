// fn main() {
//     let print =|| println!("Hello, world!");
//     print();
// }

// fn main() {
//     let print =|x| println!("Hello,{x}!");
//     print("sanju");
// }

// fn main() {
//     let sum =|x , y| println!("sum of {x} {y} = {}",x + y);
//     sum(2,3);
// }

//advance level ---------------------

fn main (){
    let result = new(2,2 ,|x,y| x+y);  //new is a fn name
      // input //clousre //fn perform
      println!("{}",result);
    
}
// <F>  define a clousre fn is defined here
// parameters and types
// fn define name as operation
// return a i32 as output
fn new< F> (x : i32 , y : i32  , operation:F)->i32
where F:Fn (i32 ,i32)->i32{
    operation(x,y)
}


#[cfg(test)]
mod tests {
    use core::num;


    #[test]
    fn it_tests_without_closure() {
        let nums = vec![1,2,3,4,5,6,7,8,9];
        let mut squares = Vec::<i32>::new();

        for num in nums {
            squares.push(num * num);
        }

        assert_eq!(vec![1,4,9,16,25,36,49,64,81], squares);
    }
    #[test]
    fn it_tests_with_closure() {
        let nums = vec![1,2,3,4,5,6,7,8,9];
        let odd_cloz = |n| n%2 != 0;
        let squares = nums.iter().map(|n| n * n).collect::<Vec<i32>>();
        let evens = nums.clone().into_iter().filter(|n| n%2 == 0).collect::<Vec<i32>>();
        let odds = nums.clone().into_iter().filter(odd_cloz).collect::<Vec<i32>>();

        assert_eq!(vec![1,4,9,16,25,36,49,64,81], squares);
        assert_eq!(vec![2,4,6,8], evens);
        assert_eq!(vec![1,3,5,7,9], odds);
    }

}


























