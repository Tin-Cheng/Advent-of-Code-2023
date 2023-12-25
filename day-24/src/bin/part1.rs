fn main() {
    println!("part1!");
    let input = include_str!("./input.txt");
    let output = part1(input, 200000000000000.0, 400000000000000.0);
    dbg!(output);
}

fn find_intersection(
    ax: f64,
    ay: f64,
    avx: f64,
    avy: f64,
    bx: f64,
    by: f64,
    bvx: f64,
    bvy: f64,
) -> Option<(f64, f64, (f64, f64))> {
    if (bvx * avy - bvy * avx) == 0.0 {
        return None;
    }
    let t2 = ((by - ay) * avx - (bx - ax) * avy) / (bvx * avy - bvy * avx);
    let t1 = (bx - ax + t2 * bvx) / avx;

    let x = ax + t1 * avx;
    let y = ay + t1 * avy;
    Some((t1, t2, (x, y)))
}

fn intersect(
    ax: f64,
    ay: f64,
    avx: f64,
    avy: f64,
    bx: f64,
    by: f64,
    bvx: f64,
    bvy: f64,
    lower_bound: f64,
    upper_bound: f64,
) -> bool {
    let intersection_point = find_intersection(ax, ay, avx, avy, bx, by, bvx, bvy);

    if intersection_point.is_some() {
        let (s1, s2, (ix, iy)) = intersection_point.unwrap();
        if s1 >= 0.0
            && s2 >= 0.0
            && ix >= lower_bound
            && iy >= lower_bound
            && ix <= upper_bound
            && iy <= upper_bound
        {
            return true;
        }
    }
    false
}

fn part1(input: &str, lower_bound: f64, upper_bound: f64) -> usize {
    let input_clean = input.replace(' ', "");
    let stones: Vec<Vec<f64>> = input_clean
        .lines()
        .map(|line: &str| {
            line.split([',', '@'])
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|s| s.parse::<f64>().unwrap())
                .collect()
        })
        .collect();
    let mut result = 0;
    for i in 0..stones.len() {
        for j in (i + 1)..stones.len() {
            if intersect(
                stones[i][0],
                stones[i][1],
                stones[i][3],
                stones[i][4],
                stones[j][0],
                stones[j][1],
                stones[j][3],
                stones[j][4],
                lower_bound,
                upper_bound,
            ) {
                result += 1
            }
        }
    }
    //println!("{:?}",stones);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_1() {
        let test_case = r#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
"#;
        let result = part1(test_case, 7.0, 27.0);
        assert_eq!(result, 2);
    }
}
