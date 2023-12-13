fn main() {
    println!("part2!");
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}
fn is_mirror(grid: &Vec<Vec<char>>, i: usize, is_vertical: bool, fixable: bool) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();

    if !is_vertical {
        let distance: usize = i.min(cols - i);
        let mut all_same = true;
        let mut one_chance = fixable;
        for j in 0..distance {
            for row in grid.iter().take(rows) {
                if row[i - j - 1] != row[i + j] {
                    if one_chance {
                        one_chance = false;
                        continue;
                    }
                    all_same = false;
                    break;
                }
            }
        }
        all_same
    } else {
        let distance: usize = i.min(rows - i);
        let mut all_same = true;
        let mut one_chance = fixable;
        for j in 0..distance {
            for c in 0..cols {
                if grid[i - j - 1][c] != grid[i + j][c] {
                    if one_chance {
                        one_chance = false;
                        continue;
                    }
                    all_same = false;
                    break;
                }
            }
        }
        all_same
    }
}

fn search_mirror(m: &str) -> usize {
    let grid: Vec<Vec<char>> = m.split('\n').map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut original_mirror: (bool, usize) = (false, 0);
    for i in 1..cols {
        if original_mirror.1 != 0 {
            break;
        }
        if is_mirror(&grid, i, false, false) {
            original_mirror = (false, i);
        }
    }
    for i in 1..rows {
        if original_mirror.1 != 0 {
            break;
        }
        if is_mirror(&grid, i, true, false) {
            original_mirror = (true, i);
        }
    }
    for i in 1..cols {
        if !original_mirror.0 && original_mirror.1 == i {
            continue;
        }
        if is_mirror(&grid, i, false, true) {
            return i;
        }
    }
    for i in 1..rows {
        if original_mirror.0 && original_mirror.1 == i {
            continue;
        }
        if is_mirror(&grid, i, true, true) {
            return i * 100;
        }
    }
    0
}

fn part2(input: &str) -> usize {
    let result: Vec<usize> = input.split("\n\n").map(search_mirror).collect();
    //println!("re {:?}", result);
    result.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_1() {
        let test_case = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let result = part2(test_case);
        assert_eq!(result, 400);
    }
}
