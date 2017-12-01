//! A solver for the [Day 1 Inverse Captcha problem](http://adventofcode.com/2017/day/1)

#![feature(slice_rotate)]
#![feature(test)]

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
    next_digits.rotate(1);

    // Add the number to the total if it is equal to the next number
    digits
        .iter()
        .zip(next_digits)
        .fold(/*||*/ 0u64, |acc, (&num, next)| {
            if num == next {
                    acc + num as u64
            } else {
                acc
            }
        })
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use self::test::Bencher;

    #[test]
    fn check_provided_test_cases() {

        // Tuple of (digits, answer)
        let test_cases = [
            (vec![1, 1, 2, 2], 3),
            (vec![1, 1, 1, 1], 4),
            (vec![1, 2, 3, 4], 0),
            (vec![9, 1, 2, 1, 2, 1, 2, 9], 9),
        ];

        for &(ref digits, answer) in test_cases.into_iter() {
            assert_eq!(answer, calculate(&mut digits.to_vec()));
        }
    }


    #[bench]
    fn benchmark(bencher: &mut Bencher) {
        bencher.iter(|| {
            let number = test::black_box(
                "1223497932759202301294722475021475739208".to_string()
            );

            // Split the number into its digits
            let mut digits: Vec<u8> = number
                .chars()
                .map(|digit| {
                    digit.to_digit(10).expect("Digits should be ascii!") as u8
                })
                .collect();

            calculate(&mut digits);
        })
    }
}
