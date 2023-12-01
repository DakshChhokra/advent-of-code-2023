fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    input.to_string()
}

#[cfg(test)]
mod tests {
  use super::*;
  
    #[test]
    fn it_works() {
        let result = part2("test");
        assert_eq!(result, "test");
    }
}