
use std::iter::zip;
fn main() {
    println!("part2!");
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

const LITERALS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
//BAD METHOD, but works for my input
const DIGITS: [&str; 9] = ["o1e", "t2o", "t3e", "f4r", "f5e", "s6x", "s7n", "e8t", "n9e"];


fn placeStringToDigit(mut input: &str) -> String{
    let mut output= String::from(input);
    let map= zip(LITERALS.iter(), DIGITS.iter());    
    for (from,to) in map{
        output = output.replace(from,to);
    };
    println!("{} : {} ",input,output);
    output
}

fn part2(input: &str) -> String {
    let numbers: Vec<u32> = input
        .lines()
        .map(|line: &str| {
            let replaced = placeStringToDigit(line);
            let num_str: Vec<char> = replaced
                .chars()
                .filter(|c| c.is_digit(10))
                .collect();
            let mut line_num =String::from("");
            match num_str.first() {
                Some(v) => line_num.push(*v),
                None => println!("The stack is empty"),
            }
            match num_str.last() {
                Some(v) => line_num.push(*v),
                None => println!("The stack is empty"),
            }
            line_num.parse().unwrap()
        })
        .collect();
        
        //.collect();
    let result:u32 = numbers.into_iter().sum();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let testcase = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        let result = part2(testcase);
        assert_eq!(result, "281".to_string());
    }
}