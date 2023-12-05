fn main() {
    println!("part2!");
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i64 {
    let data: Vec<&str> = input.split("\n\n").collect();
    //println!("data {:?}",data);
    let seeds_integers: Vec<i64> = data[0]
        .split(':')
        .collect::<Vec<&str>>()
        .last()
        .expect("should be seeds")
        .trim()
        .split(' ')
        .map(|c| c.trim().parse::<i64>())
        .filter_map(Result::ok)
        .collect();

    let seeds: Vec<Vec<i64>> = seeds_integers.chunks(2).map(|x| x.to_vec()).collect();

    //println!("seeds {:?}",seeds);

    let mut maps: Vec<Vec<Vec<i64>>> = Vec::new();

    for item in data.iter().skip(1) {
        let mut map: Vec<Vec<i64>> = item
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
        map.sort_by(|a, b| a[1].cmp(&b[1]));

        //println!("map {:?}",map);

        maps.push(map);
    }
    //println!("maps {:?}",maps);

    fn map_once(cur: Vec<i64>, map: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
        let mut result: Vec<Vec<i64>> = Vec::new();
        let mut start: i64 = cur[0];
        let end: i64 = cur[1];

        for mapping_range in map {
            let a = mapping_range[0];
            let b = mapping_range[1];
            let c = mapping_range[2];
            let diff = a - b;
            //println!("cur {:?}, map {:?}",cur,mapping_range);
            if end < b {
                result.push(vec![start, end]);
                return result;
            }
            if start < b + c {
                if b <= end && b > start {
                    result.push(vec![start, b - 1]);
                    start = b;
                }
                if end >= b + c {
                    result.push(vec![start + diff, b + c - 1 + diff]);
                    start = b + c;
                } else {
                    result.push(vec![start + diff, end + diff]);
                    return result;
                }
            }
        }
        result.push(vec![start, end]);
        //println!("cur {:?},result {:?} exit",cur, result);
        result
    }

    let mut result: i64 = std::i64::MAX;
    let mut cur_list: Vec<Vec<i64>> = Vec::new();

    for seed in seeds {
        cur_list.push(vec![seed[0], seed[0] + seed[1] - 1]);
    }
    cur_list.sort();
    //println!("start {:?}",cur_list);
    for map in &maps {
        //println!("---------------------------------------------------");
        //println!("map {:?}",map);
        let mut next_list: Vec<Vec<i64>> = Vec::new();
        for cur in cur_list {
            next_list.append(&mut map_once(cur, map));
        }
        cur_list = next_list;
        cur_list.sort();
        //println!("result {:?}",cur_list);
    }
    //println!("final result {:?}",cur_list);
    for item in cur_list {
        result = std::cmp::min(result, item[0]);
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
        let result = part2(test_case);
        assert_eq!(result, 46);
    }
}
