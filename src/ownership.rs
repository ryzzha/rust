#![warn(clippy::all, clippy::pedantic)]

use std::io;

fn process(str: &str) -> u8 {
    str.trim().parse::<u8>().expect("Please type a number!")
}

pub fn main() {
    let a = 10;
    let mut b = a;

    println!("a: {a}, b: {b}");

    b = 20;

    println!("a: {a}, b: {b}");

    let res = add_numbers(a, b);

    println!("a: {a}, b: {b}, res: {res}");

    let str1 = String::from("Hello");

    // if we reassign ptr we move the ownership and can not use str1 anymore
    // let mut str2 = str1;

    // use this method for copy value and create a new pointer
    let mut str2 = str1.clone();

    println!("str1: {str1} str2: {str2}");

    str2.replace_range(2..5, "y");

    println!("str1: {str1}, str2: {str2}");

    // if we pass as parameter base str we transfer ownership for it to func and canot use it in previous scope anymore
    // for do mut operation we must pass mutable reference

    let concatination = add_strings(&str1, " world");
    // after this operation str1 is not valid anymore
    // we can borrow str1 as immutable reference & for save ownership

    println!("str1: {str1}, str2: {str2}, concatination: {concatination}");

    let mut user_choice: String = String::new();

    io::stdin().read_line(&mut user_choice).unwrap();

    let n_choice = process(&user_choice);
    // process("test"); // error
    
    println!("Number: {n_choice}");
    println!("String: {user_choice}");
}

fn add_numbers(number_1: i32, number_2: i32) -> i32 {
    number_1 + number_2
}

fn add_strings(string_1: &str, string_2: &str) -> String {
    format!("{}{}", string_1, string_2)
}
