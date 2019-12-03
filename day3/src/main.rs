use std::fs;

// f64 is used rather than usize or even i64 so that we would not get screwed by integer division
// when determining intersection point
#[derive(Debug, PartialEq)]
struct Point{
    x: f64,
    y: f64
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self{x, y}
    }

    fn manhattan_distance_to_origin(&self) -> f64 {
        self.x + self.y
    }
}

struct WireSegment {
    start: Point,
    end: Point
}

impl WireSegment {
    fn new(start: Point, end: Point) -> Self {
        Self {
            start,
            end
        }
    }

    fn intersection(&self, other: Self) -> Option<Point> {
            let a1 = self.end.y - self.start.y;
            let b1 = self.start.x - self.end.x;
            let c1 = a1 * self.start.x + b1 * self.start.y;

            let a2 = other.end.y - other.start.y;
            let b2 = other.start.x - other.end.x;
            let c2 = a2 * other.start.x + b2 * other.start.y;

            let delta = a1 * b2 - a2 * b1;
            match delta {
                0.0 => None,
                _ => Some(Point{
                    x: (b2 * c1 - b1 * c2) / delta,
                    y: (a1 * c2 - a2 * c1) / delta,
                })
            }
    }
}

struct Wire {
    segments: Vec<WireSegment>
}

impl Wire {
    fn new_from_raw(raw_str: &str) -> Self {
        println!("new wire from {:?}", raw_str);
        Self {
            segments: vec![]
        }
    }

    fn closest_intersection_to_origin(&self, other: Self) -> Point {
        Point{
            x: 0.0,
            y: 0.0
        }
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
            6.0,
            wire1.closest_intersection_to_origin(wire2).manhattan_distance_to_origin())
    }

    #[test]
    fn it_correctly_determines_closest_intersection_for_second_input() {
        let wire1 = Wire::new_from_raw("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let wire2 = Wire::new_from_raw("U62,R66,U55,R34,D71,R55,D58,R83");

        assert_eq!(
            159.0,
            wire1.closest_intersection_to_origin(wire2).manhattan_distance_to_origin())
    }

    #[test]
    fn it_correctly_determines_closest_intersection_for_third_input() {
        let wire1 = Wire::new_from_raw("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let wire2 = Wire::new_from_raw("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");

        assert_eq!(
            135.0,
            wire1.closest_intersection_to_origin(wire2).manhattan_distance_to_origin())
    }

    #[cfg(test)]
    mod line_intersection {
        use super::*;

        #[test]
        fn it_correctly_detects_intersection() {
            let l1 = WireSegment::new(Point::new(4.0, 0.0), Point::new(6.0, 10.0));
            let l2 = WireSegment::new(Point::new(0.0, 3.0), Point::new(10.0, 7.0));
            assert_eq!(Point::new(5.0, 5.0), l1.intersection(l2).unwrap())
        }

        #[test]
        fn it_correctly_detects_no_intersection() {
            let l1 = WireSegment::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0));
            let l2 = WireSegment::new(Point::new(1.0, 2.0), Point::new(4.0, 5.0));
            assert_eq!(None, l1.intersection(l2))
        }

        #[test]
        fn it_correctly_detects_no_intersection_for_parallel_lines() {
            let l1 = WireSegment::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0));
            let l2 = WireSegment::new(Point::new(0.0, 1.0), Point::new(1.0, 2.0));
            assert_eq!(None, l1.intersection(l2))
        }
    }
}
