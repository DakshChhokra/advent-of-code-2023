use std::collections::HashMap;

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut digits_in_word: Vec<char> = Vec::new();
    let mut single_word: Vec<char> = Vec::new();

    let mut line: Vec<char> = Vec::new();

    let mut running_total = 0;

    for (i, character) in input.chars().enumerate() {
        line.push(character);
        if character == '\n' || i == input.len() - 1 {
            if !character.is_digit(10) {
                single_word.push(character);
            }

            let current_word = single_word.iter().collect::<String>();
            let mut some_vec = get_number_from_word_value(current_word.clone());
            digits_in_word.append(&mut some_vec);
            single_word = Vec::new();

            if character.is_digit(10) {
                digits_in_word.push(character);
            }

            let number_from_line = get_number_from_vector(digits_in_word);

            let line_word = line.iter().collect::<String>();

            dbg!(line_word, number_from_line);
            println!("");

            running_total += number_from_line;
            digits_in_word = Vec::new();

            line = Vec::new();
            continue;
        }
        if character.is_digit(10) {
            let current_word = single_word.iter().collect::<String>();

            let mut some_vec = get_number_from_word_value(current_word);
            digits_in_word.append(&mut some_vec);

            single_word = Vec::new();
            digits_in_word.push(character);
        } else {
            single_word.push(character);
        }
    }

    running_total.to_string()
}

fn get_number_from_word_value(word: String) -> Vec<char> {
    let mut number_hashmap: HashMap<String, char> = HashMap::new();
    number_hashmap.insert("one".to_string(), '1');
    number_hashmap.insert("two".to_string(), '2');
    number_hashmap.insert("three".to_string(), '3');
    number_hashmap.insert("four".to_string(), '4');
    number_hashmap.insert("five".to_string(), '5');
    number_hashmap.insert("six".to_string(), '6');
    number_hashmap.insert("seven".to_string(), '7');
    number_hashmap.insert("eight".to_string(), '8');
    number_hashmap.insert("nine".to_string(), '9');

    let mut numbers_in_word: HashMap<i32, char> = HashMap::new(); //idx, number
    for (key, value) in number_hashmap.clone().into_iter() {
        let occurrences: Vec<_> = word.match_indices(&key).collect();

        if !occurrences.is_empty() {
            for (index, _) in occurrences {
                numbers_in_word.insert(index as i32, value);
            }
        }
    }

    let mut number: Vec<char> = Vec::new();

    let mut i = 0;

    while i < word.len() {
        match numbers_in_word.get(&(i as i32)) {
            Some(value) => number.push(value.clone()),
            None => (),
        };
        i += 1;
    }

    number
}

fn get_number_from_vector(vector: Vec<char>) -> i32 {
    let first_digit = vector[0].to_digit(10).unwrap() as i32;
    let second_digit = vector[vector.len() - 1].to_digit(10).unwrap() as i32;
    let number = (first_digit * 10) + second_digit;
    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
4nine7oneighthm
eightwothree",
        );
        assert_eq!(result, "412");
    }
    #[test]
    fn it_works_again() {
        let result = part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
",
        );
        assert_eq!(result, "281");
    }

    #[test]
    fn it_works_again_again() {
        let result = part2("4nine7oneighthm");
        assert_eq!(result, "48");
    }

    #[test]
    fn it_works_again_again_again() {
        let result = part2("eightwothree");
        assert_eq!(result, "83");
    }

    #[test]
    fn it_works_again_again_again_again() {
        let result = part2("jnhlpkbxvm6three6seventwonegp");
        assert_eq!(result, "61");
    }

    use rstest::rstest;

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    /// this test case is from the real input
    /// it tests two overlapping numbers
    /// where the second number should succeed
    #[case("fivezg8jmf6hrxnhgxxttwoneg", 51)]
    #[case("lmscbrnlzmbqpl75ptwo64eightwoxcm", 72)]
    #[case("jnhlpkbxvm6three6seventwonegp", 61)]
    #[case("threethreetwothree", 33)]

    fn line_test(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(expected.to_string(), part2(input))
    }
}
