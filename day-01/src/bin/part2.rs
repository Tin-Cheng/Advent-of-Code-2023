fn main() {
    println!("part2!");
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn placeStringToDigit(mut input: &str) -> String{
    let mut MAPPIING: Vec<(String,String)> = Vec::new();
    //BAD METHOD
    MAPPIING.push((String::from("one"),String::from("o1e")));
    MAPPIING.push((String::from("two"),String::from("t2")));
    MAPPIING.push((String::from("three"),String::from("t3e")));
    MAPPIING.push((String::from("four"),String::from("f4r")));
    MAPPIING.push((String::from("five"),String::from("f5e")));
    MAPPIING.push((String::from("six"),String::from("s6x")));
    MAPPIING.push((String::from("seven"),String::from("s7n")));
    MAPPIING.push((String::from("eight"),String::from("e8t")));
    MAPPIING.push((String::from("nine"),String::from("n9e")));
    let mut output= String::from(input);
    for (from,to) in MAPPIING {
        output = output.replace(&String::from(from), &String::from(to));
    }
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