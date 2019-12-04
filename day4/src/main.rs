use itertools::Itertools;

const DAY4_INPUT: &str = "402328-864247";

#[derive(Debug)]
struct PasswordRange {
    min: usize,
    max: usize,
}

impl PasswordRange {
    fn new(input: &str) -> Self {
        let semiparsed_input_vec: Vec<_> = input
            .splitn(2, '-')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        assert_eq!(2, semiparsed_input_vec.len());
        Self {
            min: semiparsed_input_vec[0],
            max: semiparsed_input_vec[1],
        }
    }
}

struct Password {
    val: usize,
    val_digits: Vec<usize>,
}

impl Password {
    fn new(val: usize) -> Self {
        Password {
            val,
            val_digits: Password::val_to_digits_vec(val),
        }
    }

    fn val_to_digits_vec(val: usize) -> Vec<usize> {
        let mut digits = Vec::new();
        let mut n = val;
        while n > 9 {
            digits.push(n % 10);
            n /= 10;
        }
        digits.push(n);
        digits.reverse();
        digits
    }

    // is 3 digit -> 100 to 999 (10^2) to 10^3 - 1

    fn is_n_digit_long(&self, n: u32) -> bool {
        self.val >= 10usize.pow(n - 1) && self.val <= (10usize.pow(n) - 1)
    }

    fn is_within_range(&self, range: &PasswordRange) -> bool {
        self.val >= range.min && self.val <= range.max
    }

    fn has_same_adjacent_pair(&self) -> bool {
        self.val_digits
            .iter()
            .tuple_windows()
            .map(|(d1, d2)| d1 == d2)
            .any(|p| p)
    }

    fn is_not_decreasing(&self) -> bool {
        !self
            .val_digits
            .iter()
            .tuple_windows()
            .map(|(d1, d2)| d1 <= d2)
            .any(|p| !p)
    }
}

struct PasswordCombinations {}

impl PasswordCombinations {
    fn determine(range: &PasswordRange) -> usize {
        (range.min..=range.max) // this satisfies is_within_range
            .map(|val| Password::new(val))
            .filter(|pass| pass.is_n_digit_long(6))
            .filter(|pass| pass.has_same_adjacent_pair())
            .filter(|pass| pass.is_not_decreasing())
            .map(|_| 1)
            .sum()
    }
}

// filter 6 digit
// filter within range
// filter at least one double
// filter increasing

fn do_part1(input: &str) {
    let pwrange = PasswordRange::new(input);
    let valid_pass_count = PasswordCombinations::determine(&pwrange);
    println!("Part 1 answer: {}", valid_pass_count);
}

fn main() {
    let input = DAY4_INPUT;
    do_part1(input);
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(test)]
    mod password_validators {
        use super::*;

        #[test]
        fn number_of_digits_works_as_expected() {
            let pass1 = Password::new(1000);
            let pass2 = Password::new(1500);
            let pass3 = Password::new(9999);
            let pass4 = Password::new(999);
            let pass5 = Password::new(10000);

            assert!(pass1.is_n_digit_long(4));
            assert!(pass2.is_n_digit_long(4));
            assert!(pass3.is_n_digit_long(4));
            assert!(!pass4.is_n_digit_long(4));
            assert!(!pass5.is_n_digit_long(4));
        }

        #[test]
        fn range_works_as_expected() {
            let pass1 = Password::new(1000);
            let pass2 = Password::new(1500);
            let pass3 = Password::new(9999);
            let pass4 = Password::new(999);
            let pass5 = Password::new(10000);

            let range = PasswordRange::new("1000-9999");
            assert!(pass1.is_within_range(&range));
            assert!(pass2.is_within_range(&range));
            assert!(pass3.is_within_range(&range));
            assert!(!pass4.is_within_range(&range));
            assert!(!pass5.is_within_range(&range));
        }

        #[test]
        fn adjacent_pairs_work_as_expected() {
            let pass1 = Password::new(1);
            let pass2 = Password::new(12345);
            let pass3 = Password::new(11234);
            let pass4 = Password::new(12234);
            let pass5 = Password::new(12344);
            let pass6 = Password::new(12134);

            assert!(!pass1.has_same_adjacent_pair());
            assert!(!pass2.has_same_adjacent_pair());
            assert!(pass3.has_same_adjacent_pair());
            assert!(pass4.has_same_adjacent_pair());
            assert!(pass5.has_same_adjacent_pair());
            assert!(!pass6.has_same_adjacent_pair());
        }

        #[test]
        fn non_decreasing_works_as_expected() {
            let pass1 = Password::new(1000);
            let pass2 = Password::new(1111);
            let pass3 = Password::new(1234);
            let pass4 = Password::new(1212);

            assert!(!pass1.is_not_decreasing());
            assert!(pass2.is_not_decreasing());
            assert!(pass3.is_not_decreasing());
            assert!(!pass4.is_not_decreasing());
        }
    }
}
