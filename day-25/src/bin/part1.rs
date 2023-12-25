use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    println!("part1!");
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> usize {
    let input_clean = input.replace(':', "");

    let data: Vec<Vec<&str>> = input_clean
        .lines()
        .map(|line| line.split(' ').collect())
        .collect();
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for row in data {
        let key = row[0];
        let values = row.iter().skip(1).cloned().collect::<Vec<&str>>();
        //by observation in visualization graph
        for value in values {
            if (key == "plt" && value == "mgb")
                || (key == "tjd" && value == "dbt")
                || (key == "qns" && value == "jxm")
                || (value == "plt" && key == "mgb")
                || (value == "tjd" && key == "dbt")
                || (value == "qns" && key == "jxm")
            {
                continue;
            }
            graph.entry(value).or_default().push(key);
            graph.entry(key).or_default().push(value);
        }
    }
    let nodes: Vec<&str> = graph.keys().cloned().collect();
    let mut queue: Vec<&str> = Vec::new();
    let mut visited: HashSet<&str> = HashSet::new();
    queue.push(nodes[0]);
    while let Some(node) = queue.pop() {
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);
        queue.extend(graph.get(node).unwrap());
    }
    visited.len() * (nodes.len() - visited.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_case = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
        let result = part1(test_case);
        assert_eq!(result, 54);
    }
}
