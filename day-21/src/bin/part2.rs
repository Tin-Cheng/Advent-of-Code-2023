use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    println!("part2!");
    let input = include_str!("./input.txt");
    let output = part2(input, 500);
    dbg!(output);
}
fn get_next(
    grid: &[Vec<char>],
    gx: isize,
    gy: isize,
    rows: usize,
    cols: usize,
    r: usize,
    c: usize,
    s: usize,
) -> Vec<(isize, isize, usize, usize, usize)> {
    let mut result: Vec<(isize, isize, usize, usize, usize)> = Vec::new();
    for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let mut nr = r as isize + dr;
        let mut nc = c as isize + dc;
        let mut ngx = gx;
        let mut ngy = gy;
        if nr < 0 {
            nr = rows as isize - 1;
            ngy -= 1;
        }
        if nr == rows as isize {
            nr = 0;
            ngy += 1;
        }
        if nc < 0 {
            nc = cols as isize - 1;
            ngx -= 1;
        }
        if nc == cols as isize {
            nc = 0;
            ngx += 1;
        }
        if grid[nr as usize][nc as usize] != '#' {
            result.push((ngx, ngy, nr as usize, nc as usize, s + 1));
        }
    }
    result
}

fn simulation(grid: &Vec<Vec<char>>, steps: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut hash_set_odd: HashSet<(isize, isize, usize, usize)> = HashSet::new();
    let mut hash_set_even: HashSet<(isize, isize, usize, usize)> = HashSet::new();
    let mut hash_set_odd_corners: HashSet<(usize, usize)> = HashSet::new();
    let mut hash_set_even_corners: HashSet<(usize, usize)> = HashSet::new();
    let mut hash_set_odd_full: HashSet<(usize, usize)> = HashSet::new();
    let mut hash_set_even_full: HashSet<(usize, usize)> = HashSet::new();
    let mut start_row: usize = 0;
    let mut start_col: usize = 0;
    let mut cur_step: usize = 0;
    for (r, row) in grid.iter().enumerate().take(rows) {
        for (c, v) in row.iter().enumerate().take(cols) {
            if *v == 'S' {
                start_row = r;
                start_col = c;
                break;
            }
        }
    }
    let mut queue: VecDeque<(isize, isize, usize, usize, usize)> = VecDeque::new();
    queue.push_back((0, 0, start_row, start_col, 0));
    while let Some((gx, gy, r, c, s)) = queue.pop_front() {
        if (s % 2) == 1 && hash_set_odd.contains(&(gx, gy, r, c)) {
            continue;
        }
        if (s % 2) == 0 && hash_set_even.contains(&(gx, gy, r, c)) {
            continue;
        }
        if s > cur_step {
            cur_step = s;
        }
        if s > steps {
            break;
        }
        if (s % 2) == 1 {
            hash_set_odd.insert((gx, gy, r, c));
            if gx == 0 && gy == 0 {
                if s > 65 {
                    hash_set_odd_corners.insert((r, c));
                }
                hash_set_odd_full.insert((r, c));
            }
        } else {
            hash_set_even.insert((gx, gy, r, c));
            hash_set_odd.insert((gx, gy, r, c));
            if gx == 0 && gy == 0 {
                if s > 65 {
                    hash_set_even_corners.insert((r, c));
                }
                hash_set_even_full.insert((r, c));
            }
        }
        let neighbors = get_next(grid, gx, gy, rows, cols, r, c, s);

        for neighbor in neighbors {
            let tmp = (neighbor.0, neighbor.1, neighbor.2, neighbor.3);
            if hash_set_odd.contains(&tmp) || hash_set_even.contains(&tmp) {
                continue;
            }
            queue.push_back(neighbor);
        }
    }
    //https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
    println!(
        "odd corners {}, even corners {}, odd full {}, even full {}",
        hash_set_odd_corners.len(),
        hash_set_even_corners.len(),
        hash_set_odd_full.len(),
        hash_set_even_full.len()
    );
    let n = 202300;
    (n + 1) * (n + 1) * hash_set_odd_full.len() + n * n * hash_set_even_full.len()
        - (n + 1) * hash_set_odd_corners.len()
        + n * hash_set_even_corners.len()
}

fn part2(input: &str, steps: usize) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    simulation(&grid, steps)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_1() {
        let test_case = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;
        let result = part2(test_case, 6);
        assert_eq!(result, 16);
    }

    #[test]
    fn it_works_2() {
        let test_case = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;
        let result = part2(test_case, 10);
        assert_eq!(result, 50);
    }

    #[test]
    fn it_works_7() {
        let test_case = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;
        let result = part2(test_case, 5000);
        assert_eq!(result, 16733044);
    }
}
