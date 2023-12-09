use std::collections::{btree_map::OccupiedEntry, HashMap};

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process_line(line: &str) -> (u32, u32) {
    let card_id = line
        .split(':')
        .nth(0)
        .unwrap()
        .replace("Card ", "")
        .trim()
        .parse::<u32>()
        .unwrap();
    let card = line.split(':').nth(1).unwrap().trim();

    let card = card
        .split('|')
        .map(|sub| {
            sub.split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();

    let winning = &card[0];
    let playing = &card[1];

    let winning_numbers = winning
        .iter()
        .filter(|n| playing.contains(&n))
        .collect::<Vec<_>>();

    (card_id, winning_numbers.len() as u32)
}

fn process(input: &str) -> String {
    let scores: HashMap<u32, u32> = input.lines().map(|line| process_line(line)).fold(
        HashMap::new(),
        |mut acc, (card_id, value)| {
            acc.insert(card_id, value);
            acc
        },
    );

    let mut copies = scores.keys().fold(HashMap::new(), |mut acc, key| {
        acc.insert(key.to_owned(), 1);
        acc
    });

    let mut keys = scores.keys().collect::<Vec<_>>();
    keys.sort();
    for card in keys {
        let points = scores.get(&card).unwrap();
        let repeat = copies.get(card).unwrap();

        dbg!(card, points, repeat);
        for _ in 1..=*repeat {
            if *points > 0 {
                for n in 1..=*points {
                    let card_id = card + n;
                    *copies.entry(card_id).or_default() += 1;
                }
            }
        }
    }

    copies.values().sum::<u32>().to_string()
}

// write test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let output = "30";
        assert_eq!(process(input), output);
    }
}
