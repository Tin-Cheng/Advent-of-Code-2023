fn main() {
    println!("part1!");
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i64 {
    let sequences: Vec<Vec<i64>> = input
        .split('\n')
        .map(|line| {
            line.split(' ')
                .map(|i| {
                    i.parse::<i64>()
                        .expect("failed to convert string to integer")
                })
                .collect::<Vec<i64>>()
        })
        .collect();

    //println!("sequences {:?}",sequences);
    let mut result = 0;

    for sequence in sequences {
        let mut differences: Vec<Vec<i64>> = Vec::new();
        differences.push(sequence.clone());
        let mut all_zeros: bool = false;
        while !all_zeros {
            all_zeros = true;
            let mut difference: Vec<i64> = Vec::new();
            for i in 1..differences.last().unwrap().len() {
                difference
                    .push(differences.last().unwrap()[i] - differences.last().unwrap()[i - 1]);
                if *difference.last().unwrap() != 0_i64 {
                    all_zeros = false;
                }
            }
            differences.push(difference);
        }
        let mut next: i64 = 0;
        for i in (0..differences.len()).rev() {
            next += differences[i].last().unwrap();
        }
        result += next;
        //println!("s {:?}, n {}",sequence,next);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_case = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let result = part1(test_case);
        assert_eq!(result, 114);
    }
}
