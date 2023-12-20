use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    println!("part1!");
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> isize {
    let mut network_map: HashMap<&str, (char, Vec<&str>)> = HashMap::new();
    let mut flip_flop_map: HashMap<&str, bool> = HashMap::new();
    let mut conjunction_map: HashMap<&str, HashSet<&str>> = HashMap::new();

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
    let mut low: isize = 0;
    let mut high: isize = 0;
    for _ in 0..1000 {
        deque.push_back(("button", "broadcaster", 1));
        while !deque.is_empty() {
            let Some((source, key, pulse)) = deque.pop_front() else {
                panic!()
            };
            //println!("{} -{}-> {},",source,pulse,key);
            if pulse == 2 {
                high += 1;
            } else if pulse == 1 {
                low += 1;
            } else {
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
            for destination in destinations {
                deque.push_back((key, destination, next_pulse));
            }
        }
    }
    high * low
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_case = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        let result = part1(test_case);
        assert_eq!(result, 32000000);
    }
    #[test]
    fn it_works2() {
        let test_case = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        let result = part1(test_case);
        assert_eq!(result, 11687500);
    }
}
