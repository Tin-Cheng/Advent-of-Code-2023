use std::collections::HashMap;

fn main() {
    println!("part2!");
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}
pub fn lcm(nums: Vec<i64>) -> i64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(nums[1..].to_vec());
    println!("nums {:?}, a:{}, b:{}", nums, a, b);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
fn part2(input: &str) -> i64 {
    let input_clean = input.replace(['(', ')'], "");

    let data: Vec<&str> = input_clean.split("\n\n").collect();
    let instruction = data[0];
    //println!("ins {}",instruction);

    let network: Vec<&str> = data[1].lines().collect();
    let mut network_map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut curs: Vec<&str> = Vec::new();
    for line in network {
        //println!("{}",line);
        let line_data: Vec<&str> = line.split('=').collect();
        let key: &str = line_data[0].trim();
        if key.ends_with('A') {
            curs.push(key);
        }
        let values: Vec<_> = line_data[1].split(',').collect();
        let left: &str = values[0].trim();
        let right: &str = values[1].trim();
        network_map.insert(key, (left, right));
    }
    //println!("map {:?}",network_map);
    println!("curs {:?}", curs);

    let instruction_len: i64 = instruction.len() as i64;
    let mut answers: Vec<i64> = Vec::new(); //answer_map.values().collect();
    for _cur in curs {
        let mut cur = _cur;
        let mut steps: i64 = 0;
        while !cur.ends_with('Z') {
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
            //println!("_cur: {}, step {}, cur{}",_cur,steps,cur,);
        }
        let step = steps;
        answers.push(step);
    }

    println!("lcm: {:?}", answers);
    lcm(answers)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_case = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let result = part2(test_case);
        assert_eq!(result, 6);
    }
}
