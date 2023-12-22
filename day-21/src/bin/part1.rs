use std::collections::HashSet;

fn main() {
    println!("part1!");
    let input = include_str!("./input.txt");
    let output = part1(input, 64);
    dbg!(output);
}
fn get_next(
    grid: &[Vec<char>],
    rows: usize,
    cols: usize,
    r: usize,
    c: usize,
    s: usize,
) -> Vec<(usize, usize, usize)> {
    let mut result: Vec<(usize, usize, usize)> = Vec::new();
    for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let nr = r as isize + dr;
        let nc = c as isize + dc;
        if nr >= 0
            && nr < rows as isize
            && nc >= 0
            && nc < cols as isize
            && grid[nr as usize][nc as usize] != '#'
        {
            result.push((nr as usize, nc as usize, s + 1));
        }
    }
    result
}

fn simulation(grid: &Vec<Vec<char>>, steps: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut hash_set: HashSet<(usize, usize)> = HashSet::new();
    let mut hash_set_step: HashSet<(usize, usize, usize)> = HashSet::new();
    let mut start_row: usize = 0;
    let mut start_col: usize = 0;
    for (r, row) in grid.iter().enumerate().take(rows) {
        for (c, v) in row.iter().enumerate().take(cols) {
            if *v == 'S' {
                start_row = r;
                start_col = c;
                break;
            }
        }
    }
    let mut queue = Vec::new();
    queue.push((start_row, start_col, 0));
    while let Some((r, c, s)) = queue.pop() {
        if hash_set_step.contains(&(r, c, s)) {
            continue;
        }
        if s > steps {
            continue;
        }
        if s == steps {
            hash_set.insert((r, c));
        }
        hash_set_step.insert((r, c, s));
        let neighbors = get_next(grid, rows, cols, r, c, s);

        for neighbor in neighbors {
            queue.push(neighbor);
        }
    }
    hash_set.len()
}

fn part1(input: &str, steps: usize) -> usize {
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
        let result = part1(test_case, 6);
        assert_eq!(result, 16);
    }
}
