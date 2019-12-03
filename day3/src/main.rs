use std::fs;

struct Point(usize, usize);

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self(x, y)
    }

    fn manhattan_distance_to_origin(&self) -> usize {
        self.0 + self.1
    }
}

struct Wire {}

impl Wire {
    fn new_from_raw(raw_str: &str) -> Self {
        println!("new wire from {:?}", raw_str);
        Self {}
    }

    fn closest_intersection_to_origin(&self, other: Self) -> Point {
        Point(0, 0)
    }
}

fn do_part1(input_wires: Vec<Wire>) {
    assert_eq!(2, input_wires.len()); // as per specs
    println!("Part 1 answer: {}", 42);
}

fn read_input_file(path: &str) -> Vec<Wire> {
    fs::read_to_string(path)
        .unwrap()
        .split('\n')
        .map(|s| Wire::new_from_raw(s))
        .collect()
}

fn main() {
    let wires = read_input_file("day3.input");
    do_part1(wires);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_correctly_determines_closest_intersection_for_first_input() {
        let wire1 = Wire::new_from_raw("R8,U5,L5,D3");
        let wire2 = Wire::new_from_raw("U7,R6,D4,L4");

        assert_eq!(
            6,
            wire1.closest_intersection_to_origin(wire2).manhattan_distance_to_origin())
    }

    #[test]
    fn it_correctly_determines_closest_intersection_for_second_input() {
        let wire1 = Wire::new_from_raw("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let wire2 = Wire::new_from_raw("U62,R66,U55,R34,D71,R55,D58,R83");

        assert_eq!(
            159,
            wire1.closest_intersection_to_origin(wire2).manhattan_distance_to_origin())
    }

    #[test]
    fn it_correctly_determines_closest_intersection_for_third_input() {
        let wire1 = Wire::new_from_raw("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let wire2 = Wire::new_from_raw("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");

        assert_eq!(
            135,
            wire1.closest_intersection_to_origin(wire2).manhattan_distance_to_origin())
    }
}