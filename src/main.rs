/* This program will take 3 arguments through CLI and print the largest number among the three numbers
 
use syntax  =====>  cargo new -- arg1 arg2 arg3
*/



use std::env::{Args, args};

fn main(){
    let mut args: Args = args();

    let first = args.nth(1).unwrap().parse::<i32>().unwrap();
    let second = args.nth(0).unwrap().parse::<i32>().unwrap();
    let third = args.nth(0).unwrap().parse::<i32>().unwrap();
    
    println!("{}",largest_num(first, second, third));
}

fn largest_num(first: i32, second: i32, third: i32) -> String{

   if first > second && first > third{
        format!("first is largest number {}",first)
    }else if second > first && second > third{
        format!("second is largest number {}",second)
    }else {
        format!("third is largest number {}", third)
    }
}
