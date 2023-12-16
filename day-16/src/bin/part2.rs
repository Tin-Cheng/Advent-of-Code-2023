use std::collections::HashSet;

fn main() {
    println!("part2!");
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}
fn get_next(
    rows: usize,
    cols: usize,
    r: usize,
    c: usize,
    d: usize,
    val: char,
) -> Vec<(usize, usize, usize)> {
    let next_directions = get_directions(d, val);
    let mut result: Vec<(usize, usize, usize)> = Vec::new();
    //println!("r{}c{}nd{:?}",r,c,next_directions);
    for n_d in next_directions {
        let (dr, dc) = get_delta(n_d);
        let nr = r as isize + dr;
        let nc = c as isize + dc;
        if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
            result.push((nr as usize, nc as usize, n_d));
        }
    }
    result
}

fn get_delta(d: usize) -> (isize, isize) {
    match d {
        0 => (-1, 0),
        1 => (0, 1),
        2 => (1, 0),
        3 => (0, -1),
        _ => panic!("no direction"),
    }
}

fn get_directions(d: usize, c: char) -> Vec<usize> {
    if c == '/' {
        if d == 0 {
            return vec![1];
        }
        if d == 1 {
            return vec![0];
        }
        if d == 2 {
            return vec![3];
        }
        if d == 3 {
            return vec![2];
        }
    }
    if c == '\\' {
        if d == 0 {
            return vec![3];
        }
        if d == 1 {
            return vec![2];
        }
        if d == 2 {
            return vec![1];
        }
        if d == 3 {
            return vec![0];
        }
    }
    if c == '-' {
        if d == 0 {
            return vec![1, 3];
        }
        if d == 1 {
            return vec![1];
        }
        if d == 2 {
            return vec![1, 3];
        }
        if d == 3 {
            return vec![3];
        }
    }
    if c == '|' {
        if d == 0 {
            return vec![0];
        }
        if d == 1 {
            return vec![0, 2];
        }
        if d == 2 {
            return vec![2];
        }
        if d == 3 {
            return vec![0, 2];
        }
    }
    if c == '.' {
        return vec![d];
    }
    panic!("unknown char");
}

fn simulation(grid: &Vec<Vec<char>>, _r: usize, _c: usize, _d: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut hash_set: HashSet<(usize, usize)> = HashSet::new();
    let mut hash_set_dire: HashSet<(usize, usize, usize)> = HashSet::new();
    let mut queue = Vec::new();
    queue.push((_r, _c, _d));
    while let Some((r, c, d)) = queue.pop() {
        if hash_set_dire.contains(&(r, c, d)) {
            continue;
        }
        hash_set.insert((r, c));
        hash_set_dire.insert((r, c, d));
        let neighbors = get_next(rows, cols, r, c, d, grid[r][c]);
        for neighbor in neighbors {
            queue.push(neighbor);
        }
    }
    hash_set.len()
}

fn part2(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut result = 0;
    for r in 0..rows {
        result = result.max(simulation(&grid, r, 0, 1));
        result = result.max(simulation(&grid, r, cols - 1, 3));
    }
    for c in 0..cols {
        result = result.max(simulation(&grid, rows - 1, c, 0));
        result = result.max(simulation(&grid, 0, c, 2));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_1() {
        let test_case = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        let result = part2(test_case);
        assert_eq!(result, 51);
    }
}
