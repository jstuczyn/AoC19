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

fn do_part1(input: &str) {
    let pwrange = PasswordRange::new(input);

    println!("{:?}", pwrange);
}

fn main() {
    let input = DAY4_INPUT;
    do_part1(input);
}
