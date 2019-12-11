use itertools::Itertools;
use std::fs;

pub fn num_to_digits_vec(val: usize) -> Vec<usize> {
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

pub fn digits_vec_to_num(digits: &Vec<usize>) -> usize {
    digits
        .iter()
        .cloned()
        .fold1(|acc, x| match acc {
            0 => x,
            n => n * 10 + x,
        })
        .unwrap()
}

pub fn utf8_dec_num_repr_to_num(utf8_dec_digits: &Vec<u8>) -> isize {
    let mut possible_sign = utf8_dec_digits.iter().peekable();
    if possible_sign.peek().unwrap() == &&45 {
        0 - digits_vec_to_num(
            &utf8_dec_digits
                .iter()
                .skip(1)
                .map(|&d| (d - 48) as usize)
                .collect(),
        ) as isize
    } else {
        digits_vec_to_num(&utf8_dec_digits.iter().map(|&d| (d - 48) as usize).collect()) as isize
    }
}

pub fn parse_multiple_utf8_num_repr_lns(utf8_dec_digits_nums: &Vec<u8>) -> Vec<isize> {
    utf8_dec_digits_nums
        .split(|d| d == &10)
        .filter(|ds| !ds.is_empty())
        .map(|ds| utf8_dec_num_repr_to_num(&ds.to_vec()))
        .collect()
}

pub fn read_input_file(path: &str) -> Vec<isize> {
    fs::read_to_string(path)
        .unwrap()
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn utf8_dec_num_repr_to_num_works_for_positive_values() {
        assert_eq!(42, utf8_dec_num_repr_to_num(&vec![52, 50]))
    }

    #[test]
    fn utf8_dec_num_repr_to_num_works_for_negative_values() {
        assert_eq!(-42, utf8_dec_num_repr_to_num(&vec![45, 52, 50]))
    }

    #[test]
    fn recovering_multiple_written_values() {
        let mut out_vec = Vec::new();

        writeln!(&mut out_vec, "{}", &42).unwrap();
        writeln!(&mut out_vec, "{}", &-56).unwrap();
        writeln!(&mut out_vec, "{}", &6432).unwrap();
        writeln!(&mut out_vec, "{}", &-432).unwrap();
        writeln!(&mut out_vec, "{}", &-32432).unwrap();
        writeln!(&mut out_vec, "{}", &0).unwrap();

        assert_eq!(
            vec![42, -56, 6432, -432, -32432, 0],
            parse_multiple_utf8_num_repr_lns(&out_vec)
        )
    }
}
