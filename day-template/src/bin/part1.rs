fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    "hello".to_string()
}



#[cfg(test)]
mod tests {
       use super::*;
       use rstest::rstest;

    #[rstest]
    #[case("test", "hello")]

    fn line_test(#[case] input: &str, #[case] expected: String) {
        assert_eq!(expected.to_string(), part1(input))
    }
}
