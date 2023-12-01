fn main() {
    println!("part1!");
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let numbers: Vec<u32> = input
        .lines()
        .map(|line: &str| {
            let num_str: Vec<char> = line
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
        let testcase = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        let result = part1(testcase);
        assert_eq!(result, "142".to_string());
    }
}