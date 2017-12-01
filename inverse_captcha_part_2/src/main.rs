//! A solver for the [Day 1 Inverse Captcha problem](http://adventofcode.com/2017/day/1), part 2

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
    let mut digits: Vec<u8> = number
        .chars()
        .map(|digit| {
            digit.to_digit(10).expect("Digits should be ascii!") as u8
        })
        .collect();

    println!("Answer: {}", calculate(&mut digits))
}

fn calculate(digits: &mut Vec<u8>) -> u64 {
    let mut next_digits = digits.clone();
    next_digits.rotate(digits.len() / 2);

    // Add the number to the total if it is equal to the next number
    digits
        .iter()
        .zip(next_digits)
        .fold(0u64, |acc, (&num, next)| {
            if num == next {
                acc + num as u64
            } else {
                acc
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_provided_test_cases() {

        // Tuple of (digits, answer)
        let test_cases = [
            (vec![1, 2, 1, 2], 6),
            (vec![1, 2, 2, 1], 0),
            (vec![1, 2, 3, 4, 2, 5], 4),
            (vec![1, 2, 3, 1, 2, 3], 12),
            (vec![1, 2, 1, 3, 1, 4, 1, 5], 4)
        ];

        for &(ref digits, answer) in test_cases.into_iter() {
            assert_eq!(answer, calculate(&mut digits.to_vec()));
        }
    }
}
