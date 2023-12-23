use std::collections::HashMap;
use std::collections::HashSet;
use std::time::SystemTime;

fn main() {
    println!("part2!");
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}
fn get_next(
    grid: &[Vec<char>],
    visited: &mut HashSet<(usize, usize)>,
    rows: usize,
    cols: usize,
    r: usize,
    c: usize,
) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let nr = r as isize + dr;
        let nc = c as isize + dc;
        if !visited.contains(&(nr as usize, nc as usize))
            && nr >= 0
            && nr < rows as isize
            && nc >= 0
            && nc < cols as isize
            && grid[nr as usize][nc as usize] != '#'
        {
            result.push((nr as usize, nc as usize));
        }
    }
    result
}

fn find_closest_nodes(
    grid: &[Vec<char>],
    rows: usize,
    cols: usize,
    r: usize,
    c: usize,
) -> Vec<(usize, usize, usize)> {
    let mut result: Vec<(usize, usize, usize)> = Vec::new();
    let first_steps = get_next(grid, &mut HashSet::new(), rows, cols, r, c);
    for first_step in first_steps {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        visited.insert((r, c));
        visited.insert(first_step);
        let mut weight: usize = 1;
        let mut cur_step: (usize, usize) = first_step;
        let mut next_step = get_next(grid, &mut visited, rows, cols, cur_step.0, cur_step.1);
        while next_step.len() == 1 {
            weight += 1;
            cur_step = next_step[0];
            visited.insert(cur_step);
            next_step = get_next(grid, &mut visited, rows, cols, cur_step.0, cur_step.1);
        }
        result.push((cur_step.0, cur_step.1, weight));
    }
    result
}

fn build_paths(
    grid: &[Vec<char>],
    paths: &mut HashMap<(usize, usize), Vec<(usize, usize, usize)>>,
) {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: Vec<(usize, usize)> = Vec::new();
    queue.push((0, 1));
    while let Some((r, c)) = queue.pop() {
        if seen.contains(&(r, c)) {
            continue;
        }
        seen.insert((r, c));
        let closest_nodes = find_closest_nodes(grid, rows, cols, r, c);
        for closest_node in closest_nodes {
            if !paths.entry((r, c)).or_default().contains(&closest_node) {
                paths.entry((r, c)).or_default().push(closest_node);
            }
            if !paths
                .entry((closest_node.0, closest_node.1))
                .or_default()
                .contains(&(r, c, closest_node.2))
            {
                paths
                    .entry((closest_node.0, closest_node.1))
                    .or_default()
                    .push((r, c, closest_node.2));
            }
            queue.push((closest_node.0, closest_node.1));
        }
    }
}

fn solve(grid: &HashMap<(usize, usize), Vec<(usize, usize, usize)>>, rows: usize) -> usize {
    let mut queue: Vec<(usize, usize, usize, HashSet<(usize, usize)>)> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert((0, 1));
    queue.push((0, 1, 0, visited));
    let mut result: usize = 0;
    while let Some((r, c, s, visited)) = queue.pop() {
        if r == rows - 1 {
            result = result.max(s);
            continue;
        }
        for (nr, nc, weight) in grid.get(&(r, c)).unwrap() {
            if !visited.contains(&(*nr, *nc)) {
                let mut new_visited = visited.clone();
                new_visited.insert((*nr, *nc));
                queue.push((*nr, *nc, s + weight, new_visited));
            }
        }
    }
    result
}

fn simulation(grid: &Vec<Vec<char>>) -> usize {
    let mut paths: HashMap<(usize, usize), Vec<(usize, usize, usize)>> = HashMap::new();
    build_paths(grid, &mut paths);
    solve(&paths, grid.len())
}

fn part2(input: &str) -> usize {
    let now = SystemTime::now();
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let answer = simulation(&grid);
    match now.elapsed() {
        Ok(elapsed) => {
            println!("Duration {}ms", elapsed.as_millis());
        }
        Err(e) => {
            println!("Error: {e:?}");
        }
    }
    answer
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
        let result = part2(test_case);
        assert_eq!(result, 154);
    }
}
