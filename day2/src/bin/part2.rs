use std::cmp::max;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn game_power(rounds: Vec<&str>) -> i32 {
    let mut viable: HashMap<String, i32> = HashMap::new();
    for round in rounds {
        for cubes in round.split(',') {
            let cube_vec: Vec<&str> = cubes.trim().splitn(2, ' ').collect();
            let number = cube_vec.first().unwrap().parse::<i32>().unwrap();
            let color = cube_vec.get(1).unwrap();

            let value = match viable.get(*color) {
                Some(value) => max(*value, number),
                None => number,
            };
            viable.insert(color.to_string(), value);
        }
    }

    let mut power: i32 = 1;
    for value in viable.values() {
        power *= value;
    }
    power
}

fn process(_input: &str) -> String {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("red".to_string(), 12);
    map.insert("green".to_string(), 13);
    map.insert("blue".to_string(), 14);

    let result = _input
        .lines()
        .map(|line| line.splitn(2, ':').collect())
        .map(|line_splitted: Vec<&str>| {
            let rounds = line_splitted.get(1).unwrap().trim();
            let rounds = rounds.split(';').collect::<Vec<&str>>();
            game_power(rounds)
        })
        .inspect(|power| {
            let sum = power.to_string();
            dbg!(sum);
        })
        .sum::<i32>();

    result.to_string()
}

// write test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let output = "2286"; //include_str!("./test1.txt");
        assert_eq!(process(input), output);
    }
}
