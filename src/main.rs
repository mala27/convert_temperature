// Create a temperature converter
// Find the maths formula for the CONSTANT
// (C x 9/5) + 32 = Celcius to Fahrenheit
// (F - 32) x 5/9 = Fahrenheit to Celcius
 

use std::io;  // we need to import the user input output library

const BEGIN: f64 = 32.0;
const FRACTION: f64 = 5.0/9.0;  // this can be a constant as this value won't change, place it outside the scope

fn main() {
    println!("Enter value:");

    let mut value = String::new();

    io::stdin().read_line(&mut value)
        .expect("Sorry, cannot read line");

    let value: f64 = value.trim().parse()
        .expect("Sorry try again, enter number");

    println!("Which temperature, Celcius(c) or Fahrenheit(f)?");  // predict all the actions of the user and apply it one by one

    let mut output_type = String::new();

    io::stdin().read_line(&mut output_type)
        .expect("Sorry, cannot read line");

    let output_type = output_type.trim();  // store the output value, then pass into the program for conversion 

    let mut result = 0.0;


    if output_type == "c"{
        result = fahrenheit_to_celcius(value);
    } else if output_type == "f"{
        result = celcius_to_fahrenheit(value);
    } else {
        println!("Try again, inserted wrong temperature type");
    }

    println!("Input {}.", value);
    println!("Result {}°{}.", result, output_type);  // find which conversion type, then let the program find the correct formula
}

fn celcius_to_fahrenheit(temp:f64)->f64{
    temp / FRACTION + BEGIN
}

fn fahrenheit_to_celcius(temp:f64)->f64{
    (temp - BEGIN) * FRACTION
}
