use std::collections::HashSet;

fn main() {
    println!("part1!");
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}
fn get_next(
    grid: &[Vec<char>],
    rows: usize,
    cols: usize,
    r: usize,
    c: usize,
) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let nr = r as isize + dr;
        let nc = c as isize + dc;
        if nr >= 0
            && nr < rows as isize
            && nc >= 0
            && nc < cols as isize
            && grid[nr as usize][nc as usize] != '#'
        {
            if (grid[nr as usize][nc as usize] == '>' && (dr, dc) != (0, 1))
                || (grid[nr as usize][nc as usize] == '<' && (dr, dc) != (0, -1))
                || (grid[nr as usize][nc as usize] == '^' && (dr, dc) != (-1, 0))
                || (grid[nr as usize][nc as usize] == 'v' && (dr, dc) != (1, 0))
            {
                continue;
            }
            result.push((nr as usize, nc as usize));
        }
    }
    result
}

fn walk_bt(
    grid: &[Vec<char>],
    visited: &mut HashSet<(usize, usize)>,
    rows: usize,
    cols: usize,
    r: usize,
    c: usize,
    s: usize,
) -> usize {
    if r == rows - 1 {
        return s;
    }
    let neighbors = get_next(grid, rows, cols, r, c);
    let mut result = 0;
    for (nr, nc) in neighbors {
        if visited.contains(&(nr, nc)) {
            continue;
        }
        visited.insert((nr, nc));
        result = result.max(walk_bt(grid, visited, rows, cols, nr, nc, s + 1));
        visited.remove(&(nr, nc));
    }
    result
}

fn simulation(grid: &Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert((0, 1));
    walk_bt(grid, &mut visited, rows, cols, 0, 1, 0)
}

fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    simulation(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_1() {
        let test_case = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;
        let result = part1(test_case);
        assert_eq!(result, 94);
    }
}
