use std::fs;
use std::string;

fn get_range(input_text: &str) -> (u32, u32) {
    let range: Vec<u32> = input_text.trim()
                                    .split("-")
                                    .map(|s| s.parse().unwrap())
                                    .collect();
    (range[0], range[1])
}

fn is_six_digits(value: u32) -> bool {
    value % 100000 < value
}

fn inc_while_eq(i: usize, val: &str, rest: &str) -> usize{
    let mut ix = i;
    while  ix < rest.len() && val == &rest[ix..ix+1] {
        ix += 1;
    }
    ix
}

fn has_doubled_digit(value: u32) -> bool {
    let val_str = value.to_string();
    let mut i = 0;
    let mut prev_i = 0;
    while i < val_str.len() {
        let curr = &val_str[i..i+1];
        i = inc_while_eq(i, &curr, &val_str);
        if i-prev_i == 2 { return true; }
        prev_i = i;
    }
    return false;
}

fn digits_do_not_decrease(value: u32) -> bool {
    let mut _val = value;
    let mut left = value % 10;
    _val /= 10;
    let mut right;
    while _val > 1 {
        right = _val % 10;
        if left < right { return false; };
        _val /= 10;
        left = right;
    }

    return true;

}

fn satisfies_predicates(value: u32) -> bool {

    [is_six_digits,
     digits_do_not_decrease,
     has_doubled_digit].iter()
                      .all(|is_valid| is_valid(value))
}

fn count_passwords(low: u32, high: u32) -> u32 {
    let mut count = 0;
    for password in low..=high {
        if satisfies_predicates(password) {
            count += 1;
        }
    }

    count
}

pub fn solve() {
        const FILENAME: &str = "../challenges/2019/input_2019_4.txt";
        let input_text: string::String = fs::read_to_string(FILENAME)
                                            .expect("Something went wrong");
        let (low, high) = get_range(&input_text);

        let n_passwords_in_range = count_passwords(low, high);

        println!("\n#########################################");
        println!("Advent of Code 2019 - Day 4");
        println!("=========================================");
        println!("The number of valid passwords between {} and {} are {}", low, high, n_passwords_in_range);
        println!("#########################################");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_six_digits() {
        let num_1 = 10000;
        let num_2 = 100000;
        assert_eq!(is_six_digits(num_1), false);
        assert_eq!(is_six_digits(num_2), true);
    }

    #[test]
    fn test_has_doubled_digit() {
        let num_1 = 10234;
        let num_2 = 11234;
        let num_3 = 111234;
        let num_4 = 122233;
        let num_5 = 123444;
        assert_eq!(has_doubled_digit(num_1), false);
        assert_eq!(has_doubled_digit(num_2), true);
        assert_eq!(has_doubled_digit(num_3), false);
        assert_eq!(has_doubled_digit(num_4), true);
        assert_eq!(has_doubled_digit(num_5), false);
    }

    #[test]
    fn test_digits_do_not_decrease() {
        let num_1 = 122342;
        let num_2 = 122345;
        assert_eq!(digits_do_not_decrease(num_1), false);
        assert_eq!(digits_do_not_decrease(num_2), true);
    }

    #[test]
    fn test_satisfies_predicates() {
        let num_1 = 111122;
        let num_2 = 223450;
        let num_3 = 123789;
        assert_eq!(satisfies_predicates(num_1), true);
        assert_eq!(satisfies_predicates(num_2), false);
        assert_eq!(satisfies_predicates(num_3), false);
    }
}
