fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {

    let mut digits_in_word: Vec<char> = Vec::new();

    let mut running_total = 0;
    for (i, character) in input.chars().enumerate() {
        if character == '\n' || i == input.len()-1 {
            dbg!(get_number_from_vector(digits_in_word.clone()));
            dbg!(digits_in_word.clone());
            running_total += get_number_from_vector(digits_in_word);
            digits_in_word = Vec::new();
            continue;
        }
        if character.is_digit(10) {
            digits_in_word.push(character);
        }
    }
    
    running_total.to_string()
}

fn get_number_from_vector(vector: Vec<char>) -> i32 {
    let first_digit = vector[0].to_digit(10).unwrap() as i32;
    let second_digit = vector[vector.len()-1].to_digit(10).unwrap() as i32;
    let number = (first_digit* 10) + second_digit;
    number
}

#[cfg(test)]
mod tests {
  use super::*;

    #[test]
    fn it_works() {
        let result = part1("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");
        assert_eq!(result, "142");
    }
}