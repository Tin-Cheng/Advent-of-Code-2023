use std::collections::HashSet;

use priority_queue::PriorityQueue;

fn main() {
    println!("part1!");
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}
fn get_delta(d: usize) -> (isize, isize) {
    match d {
        0 => (-1, 0),
        1 => (0, 1),
        2 => (1, 0),
        3 => (0, -1),
        _ => panic!("unknown direction"),
    }
}
fn get_next_directions(d: usize) -> Vec<usize> {
    match d {
        0 => vec![3, 1],
        1 => vec![0, 2],
        2 => vec![1, 3],
        3 => vec![0, 2],
        _ => panic!("unknown direction"),
    }
}
fn get_next(
    grid: &[Vec<usize>],
    rows: usize,
    cols: usize,
    cost: usize,
    r: usize,
    c: usize,
    d: usize,
) -> Vec<(usize, usize, usize, usize)> {
    let mut result: Vec<(usize, usize, usize, usize)> = Vec::new();
    let (dr, dc) = get_delta(d);
    let next_directions = get_next_directions(d);
    let mut new_cost = cost;
    for i in 1..4 {
        let nr = r as isize + dr * i;
        let nc = c as isize + dc * i;
        if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
            new_cost += grid[nr as usize][nc as usize];
            for next_direction in &next_directions {
                result.push((new_cost, nr as usize, nc as usize, *next_direction));
            }
        }
    }
    result
}
fn part1pq(grid: &Vec<Vec<usize>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut hash_set_dire: HashSet<(usize, usize, usize)> = HashSet::new();
    let mut queue = PriorityQueue::new();
    queue.push((0, 0, 0, 1), 0);
    queue.push((0, 0, 0, 2), 0);
    while let Some(((cost, r, c, d), _priority)) = queue.pop() {
        if hash_set_dire.contains(&(r, c, d)) {
            continue;
        }
        hash_set_dire.insert((r, c, d));
        if r == rows - 1 && c == cols - 1 {
            return cost;
        }
        let neighbors = get_next(grid, rows, cols, cost, r, c, d);
        for neighbor in neighbors {
            queue.push(neighbor, -(neighbor.0 as isize));
        }
    }
    panic!("should arrive to end");
}

fn part1(input: &str) -> usize {
    let grid: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|digit| digit as usize)
                .collect()
        })
        .collect();
    part1pq(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_1() {
        let test_case = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let result = part1(test_case);
        assert_eq!(result, 102);
    }
}
