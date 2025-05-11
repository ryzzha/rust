// use std::io;

mod types;
mod binary_search;

// const C: f32 = 32.0;

// fn c_to_f(celsius: f32) -> f32 {
//     (celsius * 9.0 / 5.0) + C
// }

// fn f_to_c(fahrenheit: f32) -> f32 {
//     (fahrenheit - C) * 5.0 / 9.0
// } 

// fn convert(temperature: f32, conversion_type: u8) -> Option<f32> {
//     match conversion_type {
//         1 => Some(c_to_f(temperature)),
//         2 => Some(f_to_c(temperature)),
//         _ => None,
//     }
// }

fn main() { 
    // println!("Select the conversion type: \n 1 -> c_to_f \n 2 -> f_to_c");

    // let mut conversion_choice = String::new();

    // io::stdin().read_line(&mut conversion_choice).unwrap();

    // let conversion_type = conversion_choice.trim().parse::<u8>().expect("Invalid input");

    // println!("Enter the temperature to convert:");

    // let mut temperature_input = String::new();

    // io::stdin().read_line(&mut temperature_input).unwrap();

    // let temperature = temperature_input.trim().parse::<f32>().expect("Invalid input");

    // let result = convert(temperature, conversion_type);

    // match result {
    //     Some(result) => println!("The result of conversion is: {result}"),
    //     None => println!("Unknown conversion requested!"),
    // };

    // types::main()
    binary_search::main()
}