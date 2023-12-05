fn main() {
    println!("part1!");
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i64 {
    let data: Vec<&str> = input.split("\n\n").collect();
    //println!("data {:?}",data);
    let seeds: Vec<i64> = data[0]
        .split(':')
        .collect::<Vec<&str>>()
        .last()
        .expect("should be seeds")
        .trim()
        .split(' ')
        .map(|c| c.trim().parse::<i64>())
        .filter_map(Result::ok)
        .collect();
    //println!("seeds {:?}",seeds);

    let mut maps: Vec<Vec<Vec<i64>>> = Vec::new();

    for item in data.iter().skip(1) {
        let map: Vec<Vec<i64>> = item
            .split(':')
            .collect::<Vec<&str>>()
            .last()
            .expect("should be mapping numbers")
            .trim()
            .split('\n')
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| {
                s.trim()
                    .split(' ')
                    .map(|c| c.parse::<i64>())
                    .filter_map(Result::ok)
                    .collect()
            })
            .collect();

        maps.push(map);
    }
    //println!("maps {:?}",maps);

    fn map_once(cur: i64, map: &Vec<Vec<i64>>) -> i64 {
        for mapping_range in map {
            let a = mapping_range[0];
            let b = mapping_range[1];
            let c = mapping_range[2];
            if (b..(b + c)).contains(&cur) {
                //println!("cur {:?}, map {:?}, ans {}",cur,mapping_range,cur + a - b);
                return cur + a - b;
            }
        }
        //println!("cur {:?}, not in range",cur);
        cur
    }

    let mut result: i64 = std::i64::MAX;
    for seed in seeds {
        let mut cur = seed;
        //println!("work on seed {}",seed);
        for map in &maps {
            cur = map_once(cur, map);
        }
        //seed = cur;
        //println!("seed {}, result {}",seed, cur);
        result = std::cmp::min(result, cur);
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_case = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = part1(test_case);
        assert_eq!(result, 35);
    }
}
