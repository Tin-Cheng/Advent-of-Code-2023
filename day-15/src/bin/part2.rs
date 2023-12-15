use std::collections::HashMap;

fn main() {
    println!("part2!");
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn hash(s: &str) -> (String, usize, usize, char) {
    let mut cur: usize = 0;
    let s_arr: Vec<char> = s.chars().collect();
    for i in 0..s_arr.len() {
        if s_arr[i] == '=' {
            let label = &s[0..i];
            let val: usize = s_arr[s_arr.len() - 1] as usize - '0' as usize;
            return (label.to_string(), cur, val, s_arr[i]);
        }
        if s_arr[i] == '-' {
            let label = &s[0..i];
            return (label.to_string(), cur, 0, s_arr[i]);
        }
        let ascii_value: usize = s_arr[i] as usize;
        cur += ascii_value;
        cur *= 17;
        cur %= 256;
    }
    ("".to_string(), 0, 0, ' ')
}

fn part2(input: &str) -> usize {
    let mut boxes: Vec<Vec<String>> = Vec::new();
    let mut hash_map: HashMap<String, usize> = HashMap::new();
    for _ in 0..256 {
        let inner_vector: Vec<String> = Vec::new();
        boxes.push(inner_vector);
    }
    *hash_map.entry((&input).to_string()).or_insert(0) = 12345;
    input.split(',').for_each(|s: &str| {
        let (label, index, val, symbol) = hash(s);
        if symbol == '=' {
            if !boxes[index].contains(&label) {
                boxes[index].push(label.clone());
            }
            *hash_map.entry(label.clone()).or_insert(0) = val;
        }
        if symbol == '-' {
            if let Some(label_index) = boxes[index].iter().position(|s| s == &label) {
                boxes[index].remove(label_index);
            }
        }
    });

    let mut result: usize = 0;
    for (i, _box) in boxes.iter().enumerate().take(256) {
        for (j, v) in _box.iter().enumerate() {
            //println!("i {}, j {}, label {}, val {}",i+1,j+1,&boxes[i][j],hash_map.get(&boxes[i][j]).unwrap());
            result += (i + 1) * (j + 1) * hash_map.get(v).unwrap();
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_1() {
        let test_case = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let result = part2(test_case);
        assert_eq!(result, 145);
    }
}
