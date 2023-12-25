//https://github.com/arthomnix/aoc23/blob/master/src/days/day24.rs

use z3::{Config, Context, Solver};
use z3::ast::{Ast, Int};

fn main() {
    println!("part2!");
    let input = include_str!("./input.txt");
    let output = part2(input);
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

fn part2(input: &str) -> i64 {
    let input_clean = input.replace(' ', "");
    let stones: Vec<Vec<i64>> = input_clean
        .lines()
        .map(|line: &str| {
            line.split([',', '@'])
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|s| s.parse::<i64>().unwrap())
                .collect()
        })
        .collect();
    
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let px = Int::new_const(&ctx, "px");
    let py = Int::new_const(&ctx, "py");
    let pz = Int::new_const(&ctx, "pz");
    let vx = Int::new_const(&ctx, "vx");
    let vy = Int::new_const(&ctx, "vy");
    let vz = Int::new_const(&ctx, "vz");

    for stone in stones {
        let pxn = Int::from_i64(&ctx, stone[0]);
        let pyn = Int::from_i64(&ctx, stone[1]);
        let pzn = Int::from_i64(&ctx, stone[2]);
        let vxn = Int::from_i64(&ctx, stone[3]);
        let vyn = Int::from_i64(&ctx, stone[4]);
        let vzn = Int::from_i64(&ctx, stone[5]);
        let tn = Int::fresh_const(&ctx, "t");

        solver.assert(&(&pxn + &vxn * &tn)._eq(&(&px + &vx * &tn)));
        solver.assert(&(&pyn + &vyn * &tn)._eq(&(&py + &vy * &tn)));
        solver.assert(&(&pzn + &vzn * &tn)._eq(&(&pz + &vz * &tn)));
    }

    solver.check();
    println!("ready to get model, {}",solver.to_string());
    let model = solver.get_model().unwrap();
    let x = model.get_const_interp(&px).unwrap().as_i64().unwrap();
    let y = model.get_const_interp(&py).unwrap().as_i64().unwrap();
    let z = model.get_const_interp(&pz).unwrap().as_i64().unwrap();
    println!("{},{},{}",x,y,z);
    x + y +z 
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
        let result = part2(test_case);
        assert_eq!(result, 47);
    }
}
