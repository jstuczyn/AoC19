use itertools::Itertools;

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
