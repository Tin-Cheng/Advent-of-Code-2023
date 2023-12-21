use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    println!("part2!");
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

pub fn lcm(nums: Vec<isize>) -> isize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(nums[1..].to_vec());
    //println!("nums {:?}, a:{}, b:{}", nums, a, b);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: isize, b: isize) -> isize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
fn part2(input: &str) -> isize {
    let mut network_map: HashMap<&str, (char, Vec<&str>)> = HashMap::new();
    let mut flip_flop_map: HashMap<&str, bool> = HashMap::new();
    let mut conjunction_map: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut answers: HashMap<&str, isize> = HashMap::new();

    let input_clean = input.replace(' ', "");
    input_clean.lines().for_each(|line| {
        let data: Vec<&str> = line.split("->").collect();
        let key: &str = if data[0] == "broadcaster" {
            data[0]
        } else {
            &data[0][1..data[0].len()]
        };
        let ty: char = data[0].chars().next().unwrap();
        match ty {
            '%' => {
                flip_flop_map.insert(key, false);
            }
            '&' => {
                let hs = HashSet::new();
                conjunction_map.insert(key, hs);
            }
            'b' => {}
            _ => panic!("invalid type"),
        };
        let destinations = data[1].split(',').collect();
        network_map.insert(key, (ty, destinations));
    });

    for (key, (_, destinations)) in network_map.iter() {
        for destination in destinations {
            if !network_map.contains_key(destination) {
                continue;
            }
            let (ty, _) = network_map.get(destination).unwrap();
            if ty == &'&' {
                let hs = &mut conjunction_map.get_mut(destination).unwrap();
                hs.insert(key);
            }
        }
    }

    let mut deque: VecDeque<(&str, &str, usize)> = VecDeque::new();
    let mut count: isize = 0;
    while answers.len() < 4 {
        count += 1;
        deque.push_back(("button", "broadcaster", 1));
        while !deque.is_empty() {
            let Some((source, key, pulse)) = deque.pop_front() else {
                panic!()
            };

            if pulse == 0 {
                continue;
            }
            if !network_map.contains_key(key) {
                continue;
            }
            let (ty, destinations) = network_map.get(key).unwrap();
            let next_pulse: usize;
            match ty {
                '%' => {
                    if pulse == 1 {
                        let state = flip_flop_map.get_mut(key).unwrap();
                        if *state {
                            *state = !*state;
                            next_pulse = 1;
                        } else {
                            *state = !*state;
                            next_pulse = 2;
                        }
                    } else {
                        next_pulse = 0;
                    }
                }
                '&' => {
                    let hs = &mut conjunction_map.get_mut(key).unwrap();
                    if pulse == 2 {
                        hs.remove(source);
                    } else {
                        hs.insert(source);
                    }
                    if hs.is_empty() {
                        next_pulse = 1;
                    } else {
                        next_pulse = 2;
                    }
                }
                'b' => {
                    next_pulse = pulse;
                }
                _ => {
                    panic!("unknown type");
                }
            };
            if next_pulse == 0 {
                continue;
            }
            //observe from input
            if (key == "xl" || key == "ln" || key == "xp" || key == "gp")
                && next_pulse == 2
                && !answers.contains_key(key)
            {
                answers.insert(key, count);
            }

            for destination in destinations {
                deque.push_back((key, destination, next_pulse));
            }
        }
    }
    let mut answer_vec: Vec<isize> = Vec::new();
    for (_, value) in answers.iter() {
        answer_vec.push(*value);
    }
    lcm(answer_vec)
}
