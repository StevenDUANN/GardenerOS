#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
const RESULT:u32 = 1+1;

#[no_mangle]
fn main() -> i32 {
    println!("Welcome to my first linux program!");
    println!("It can calculate the result of '1+1'!");

    println!("1+1={}",RESULT);
    0
}