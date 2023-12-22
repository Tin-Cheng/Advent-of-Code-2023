use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    println!("part1!");
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}
fn check_fallible(
    space: &HashMap<(usize, usize, usize), usize>,
    above: &mut HashMap<usize, Vec<usize>>,
    below: &mut HashMap<usize, HashSet<isize>>,
    i: usize,
    x_y_coord: Vec<(usize, usize)>,
    z: usize,
) -> bool {
    let mut result = true;
    for (x, y) in x_y_coord {
        if space.contains_key(&(x, y, z - 1)) {
            let b = space.get(&(x, y, z - 1)).unwrap();
            result = false;
            above.entry(*b).or_default().push(i);
            below.entry(i).or_default().insert(*b as isize);
        }
    }
    if z == 1 {
        below.entry(i).or_default().insert(-1);
    }
    result
}

fn part1(input: &str) -> usize {
    let mut blocks: Vec<((usize, usize, usize), (usize, usize, usize))> = Vec::new();
    let mut space: HashMap<(usize, usize, usize), usize> = HashMap::new();
    let mut above: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut below: HashMap<usize, HashSet<isize>> = HashMap::new();
    for line in input.lines() {
        let nums = line
            .split([',', '~'])
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        blocks.push(((nums[0], nums[1], nums[2]), (nums[3], nums[4], nums[5])));
    }
    blocks.sort_by_cached_key(|b| b.0 .2.min(b.1 .2));
    for (i, &((x1, y1, z1), (x2, y2, z2))) in blocks.iter().enumerate() {
        let mut cur_z = z1.min(z2);
        let height = z1.max(z2) - cur_z;
        let x_y_coord: Vec<(usize, usize)> = if x1 == x2 {
            (y1..=y2).map(|y| (x1, y)).collect()
        } else {
            (x1..=x2).map(|x| (x, y1)).collect()
        };
        while cur_z > 1
            && check_fallible(&space, &mut above, &mut below, i, x_y_coord.clone(), cur_z)
        {
            cur_z -= 1;
        }
        for z in cur_z..=cur_z + height {
            for (x, y) in &x_y_coord {
                space.insert((*x, *y, z), i);
            }
        }
        println!(
            "end   {}: {}, {}, {} - {}, {}, {}",
            i,
            x1,
            y1,
            cur_z,
            x2,
            y2,
            cur_z + height
        );
    }
    let mut result: usize = 0;
    for i in 0..blocks.len() {
        let mut safe = true;
        if above.contains_key(&i) {
            let blocks_above = above.get(&i).unwrap();
            for block_above in blocks_above {
                if below.get(block_above).unwrap().len() == 1 {
                    safe = false;
                    break;
                }
            }
        }
        if safe {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_1() {
        let test_case = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#;
        let result = part1(test_case);
        assert_eq!(result, 5);
    }
}
