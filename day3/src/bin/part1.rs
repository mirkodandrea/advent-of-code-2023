use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(_input: &str) -> String {
    todo!()
}

// write test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "";
        let output = ""; //include_str!("./test1.txt");
        assert_eq!(process(input), output);
    }
}
