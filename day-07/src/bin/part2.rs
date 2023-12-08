use std::collections::HashMap;

fn main() {
    println!("part2!");
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i64 {
    let hands: Vec<Vec<&str>> = input
        .lines()
        .map(|s| s.split(' ').collect::<Vec<&str>>())
        .collect();

    let sort_order = "J23456789TQKA";

    let mut five_of_a_kind: Vec<Vec<&str>> = Vec::new();
    let mut four_of_a_kind: Vec<Vec<&str>> = Vec::new();
    let mut full_house: Vec<Vec<&str>> = Vec::new();
    let mut three_of_a_kind: Vec<Vec<&str>> = Vec::new();
    let mut two_pair: Vec<Vec<&str>> = Vec::new();
    let mut one_pair: Vec<Vec<&str>> = Vec::new();
    let mut high_card: Vec<Vec<&str>> = Vec::new();

    for hand in hands{
        let cards = hand[0];
        let mut counts: HashMap<char,i8> = HashMap::new();
        counts.entry('J').or_insert(0);
        for card in cards.chars() {
            *counts.entry(card).or_insert(0) += 1;
        }
        let mut counts_vec: Vec<_> = counts.iter().collect::<Vec<_>>();//.sort();
        counts_vec.sort_by_key(|x| -1 * x.1);

        let j_card: i8 = *counts.get(&'J').unwrap();
        let mut top: i8 = *counts_vec[0].1;
        let mut second: i8 = if counts_vec.len() > 1{ *counts_vec[1].1 }else{ 0 };
        if counts_vec[0].0 == &'J' && counts_vec.len() > 1{
            top += second;
            if counts_vec.len() > 2{
                second = *counts_vec[2].1;
            }
        }else if counts_vec[0].0 != &'J'{
            top += j_card;
        }
        if top == 5 as i8 {
            five_of_a_kind.push(hand);
        }else if top == 4 as i8{
            four_of_a_kind.push(hand);
        }else if top == 3 as i8 && second == 2 as i8 {
            full_house.push(hand);
        }else if top == 3 as i8 {
            three_of_a_kind.push(hand);
        }else if top == 2 as i8 && second == 2 as i8 {
            two_pair.push(hand);
        }else if top == 2 as i8 {
            one_pair.push(hand);
        }else{
            high_card.push(hand);
        }
    }

    five_of_a_kind.sort_by_key(|x| 
        (
            sort_order.find(x[0].chars().nth(0).unwrap()),
            sort_order.find(x[0].chars().nth(1).unwrap()),
            sort_order.find(x[0].chars().nth(2).unwrap()),
            sort_order.find(x[0].chars().nth(3).unwrap()),
            sort_order.find(x[0].chars().nth(4).unwrap())
        )
    );
    four_of_a_kind.sort_by_key(|x| 
        (
            sort_order.find(x[0].chars().nth(0).unwrap()),
            sort_order.find(x[0].chars().nth(1).unwrap()),
            sort_order.find(x[0].chars().nth(2).unwrap()),
            sort_order.find(x[0].chars().nth(3).unwrap()),
            sort_order.find(x[0].chars().nth(4).unwrap())
        )
    );
    full_house.sort_by_key(|x| 
        (
            sort_order.find(x[0].chars().nth(0).unwrap()),
            sort_order.find(x[0].chars().nth(1).unwrap()),
            sort_order.find(x[0].chars().nth(2).unwrap()),
            sort_order.find(x[0].chars().nth(3).unwrap()),
            sort_order.find(x[0].chars().nth(4).unwrap())
        )
    );

    three_of_a_kind.sort_by_key(|x| 
        (
            sort_order.find(x[0].chars().nth(0).unwrap()),
            sort_order.find(x[0].chars().nth(1).unwrap()),
            sort_order.find(x[0].chars().nth(2).unwrap()),
            sort_order.find(x[0].chars().nth(3).unwrap()),
            sort_order.find(x[0].chars().nth(4).unwrap())
        )
    );

    two_pair.sort_by_key(|x| 
        (
            sort_order.find(x[0].chars().nth(0).unwrap()),
            sort_order.find(x[0].chars().nth(1).unwrap()),
            sort_order.find(x[0].chars().nth(2).unwrap()),
            sort_order.find(x[0].chars().nth(3).unwrap()),
            sort_order.find(x[0].chars().nth(4).unwrap())
        )
    );

    one_pair.sort_by_key(|x| 
        (
            sort_order.find(x[0].chars().nth(0).unwrap()),
            sort_order.find(x[0].chars().nth(1).unwrap()),
            sort_order.find(x[0].chars().nth(2).unwrap()),
            sort_order.find(x[0].chars().nth(3).unwrap()),
            sort_order.find(x[0].chars().nth(4).unwrap())
        )
    );

    high_card.sort_by_key(|x| 
        (
            sort_order.find(x[0].chars().nth(0).unwrap()),
            sort_order.find(x[0].chars().nth(1).unwrap()),
            sort_order.find(x[0].chars().nth(2).unwrap()),
            sort_order.find(x[0].chars().nth(3).unwrap()),
            sort_order.find(x[0].chars().nth(4).unwrap())
        )
    );
    
    let mut result: i64 = 0;
    let mut rank: i64 = 1;
    for hand in high_card{
        result += rank * hand[1].parse::<i64>().unwrap();
        rank += 1;
    }
    for hand in one_pair{
        result += rank * hand[1].parse::<i64>().unwrap();
        rank += 1;
    }
    for hand in two_pair{
        result += rank * hand[1].parse::<i64>().unwrap();
        rank += 1;
    }
    for hand in three_of_a_kind{
        result += rank * hand[1].parse::<i64>().unwrap();
        rank += 1;
    }
    for hand in full_house{
        result += rank * hand[1].parse::<i64>().unwrap();
        rank += 1;
    }
    for hand in four_of_a_kind{
        result += rank * hand[1].parse::<i64>().unwrap();
        rank += 1;
    }
    for hand in five_of_a_kind{
        result += rank * hand[1].parse::<i64>().unwrap();
        rank += 1;
    }
    

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_case = 
"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let result = part2(test_case);
        assert_eq!(result, 5905);
    }
}
