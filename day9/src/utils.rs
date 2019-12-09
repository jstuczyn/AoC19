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

pub fn read_input_file(path: &str) -> Vec<isize> {
    fs::read_to_string(path)
        .unwrap()
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect()
}
