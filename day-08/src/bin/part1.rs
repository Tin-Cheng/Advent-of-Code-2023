use std::collections::HashMap;

fn main() {
    println!("part1!");
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i64 {
    let input_clean = input.replace(['(', ')'], "");

    let data: Vec<&str> = input_clean.split("\n\n").collect();
    let instruction = data[0];
    println!("ins {}", instruction);

    let network: Vec<&str> = data[1].lines().collect();
    let mut network_map: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in network {
        println!("{}", line);
        let line_data: Vec<&str> = line.split('=').collect();
        let key: &str = line_data[0].trim();
        let values: Vec<_> = line_data[1].split(',').collect();
        let left: &str = values[0].trim();
        let right: &str = values[1].trim();
        network_map.insert(key, (left, right));
    }
    //println!("map {:?}",network_map);

    let mut steps: i64 = 0;
    let mut cur: &str = "AAA";
    let instruction_len: i64 = instruction.len() as i64;
    while cur != "ZZZ" {
        if instruction
            .chars()
            .nth((steps % instruction_len).try_into().unwrap())
            == Some('L')
        {
            cur = network_map.get(cur).unwrap().0;
        } else {
            cur = network_map.get(cur).unwrap().1;
        }

        steps += 1;
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_case = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let result = part1(test_case);
        assert_eq!(result, 6);
    }
}
