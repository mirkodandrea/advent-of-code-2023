use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn get_symbols_positions(lines: &[&str]) -> Vec<bool> {
    let first_line = lines[0];
    let mut acc: Vec<bool> = vec![false; first_line.len()];

    for line in lines {
        let are_symbols: Vec<bool> = line
            .chars()
            .map(|c| !(c.is_digit(10) || c == '.'))
            .collect();
        for i in 0..acc.len() {
            acc[i] = acc[i] | are_symbols[i]
        }
    }
    acc
}

fn process_line(line: &str, symbols: Vec<bool>) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();

    let mut num: Vec<char> = Vec::new();
    let mut keep = false;

    for (c, chr) in line.chars().enumerate() {
        if chr.is_numeric() {
            num.push(chr);
            //check symbols before and and after
            let start = if c == 0 { 0 } else { c - 1 };
            let end = if (c + 2) > symbols.len() {
                symbols.len()
            } else {
                c + 2
            };

            keep = symbols[start..end]
                .into_iter()
                .fold(keep, |_keep, s| _keep || *s);
        } else {
            if num.len() > 0 && keep {
                let s: String = num.iter().collect();
                dbg!(&s);

                let value = s.parse::<u32>().unwrap();
                numbers.push(value);

                keep = false;
            }

            num.clear();
        }
    }
    if num.len() > 0 && keep {
        let s: String = num.iter().collect();

        let value = s.parse::<u32>().unwrap();
        numbers.push(value);

        num.clear();
    }

    numbers
}

fn process(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut total = 0;
    for (idx, line) in lines.iter().enumerate() {
        let line = line.trim();
        dbg!(line);
        let start = if idx < 1 { 0 } else { idx - 1 };
        let end = if (idx + 2) > (lines.len()) {
            lines.len()
        } else {
            idx + 2
        };

        let symbols = get_symbols_positions(&lines[start..end]);

        let numbers = process_line(line, symbols);

        for num in numbers {
            total += num;
        }
    }
    total.to_string()
}

// write test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let output = "4361";
        assert_eq!(process(input), output);
    }

    #[test]
    fn test_last_lines() {
        let input = ".....86....&........702........./..*.......363.........................=630.737#............%...........................................259.
......*....256.......*.......+..57..806.......................591*............................*348....829...................+460............
.....................244....6.....................................789......................687..............................................";
        let lines: Vec<&str> = input.lines().collect();
        let symbols = get_symbols_positions(&lines[0..3]);
        let numbers = process_line(lines[1], symbols);
        dbg!(&numbers);
        let output = vec![256, 57, 806, 591, 348, 460];
        assert_eq!(numbers, output);
    }
}
