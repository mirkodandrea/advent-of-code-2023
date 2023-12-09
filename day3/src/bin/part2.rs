fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let mut total = 0;

    for (idx, line) in lines.iter().enumerate() {
        let line = line.trim();

        // check if '*' is in the line
        let gear_indexes = line
            .chars()
            .enumerate()
            .filter_map(|(i, c)| if c == '*' { Some(i) } else { None })
            .collect::<Vec<usize>>();
        if gear_indexes.len() == 0 {
            continue;
        }

        let start = if idx < 1 { 0 } else { idx - 1 };
        let end = if (idx + 2) > (lines.len()) {
            lines.len()
        } else {
            idx + 2
        };

        let slice = &lines[start..end];

        for gear_index in gear_indexes {
            let mut pair: Vec<u32> = Vec::new();
            for s in slice {
                let chrs = s.chars().collect::<Vec<char>>();

                let mut prev: Vec<char> = Vec::new();
                let mut next: Vec<char> = Vec::new();

                for i in (0..gear_index).rev() {
                    let chr = chrs[i];
                    if chr.is_numeric() {
                        prev.push(chr);
                    } else {
                        break;
                    }
                }
                for i in gear_index + 1..chrs.len() {
                    let chr = chrs[i];
                    if chr.is_numeric() {
                        next.push(chr);
                    } else {
                        break;
                    }
                }

                let prev = prev.iter().rev().copied().collect::<Vec<char>>();

                let mut combined = prev.clone();
                combined.push(chrs[gear_index]);
                combined.extend(next.clone());

                let combined: String = combined
                    .iter()
                    .map(|x| match x {
                        '0'..='9' => *x,
                        _ => ' ',
                    })
                    .collect();

                let split = combined
                    .trim()
                    .split(' ')
                    .filter_map(|s| s.parse::<u32>().ok())
                    .collect::<Vec<u32>>();

                for s in split {
                    pair.push(s);
                }
            }
            if pair.len() > 2 {
                dbg!("something is wrong", &pair);
            }
            if pair.len() == 2 {
                dbg!(&pair, idx, gear_index);
                total += pair[0] * pair[1];
            }
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
        let output = "467835";
        assert_eq!(process(input), output);
    }
}
