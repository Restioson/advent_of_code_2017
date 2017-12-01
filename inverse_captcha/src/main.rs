//! A solver for the [Day 1 Inverse Captcha problem](http://adventofcode.com/2017/day/1)

#![feature(slice_rotate)]

#[macro_use]
extern crate scan_rules;

use scan_rules::scanner::Number;

fn main() {

    print!("Enter the captcha input: ");

    // Read in a number using scan_rules try_readln! macro
    let number = try_readln! { (let number: Number<String>) => number }
        .expect("A number must be inputted");

    // Split the number into its digits
    let digits: Vec<u8> = number.chars()
        .map(|digit| digit.to_digit(10).expect("Digits should be ascii!") as u8)
        .collect();

    // Add the number to the total if it is equal to the next number
    let mut next_digits = digits.clone();
    next_digits.rotate(1);

    let answer = digits.iter()
        .zip(next_digits)
        .fold(0u64, |acc, (&num, next)| {
            if num == next {
                acc + num as u64
            } else {
                acc
            }
        });

    println!("Answer: {}", answer)
}