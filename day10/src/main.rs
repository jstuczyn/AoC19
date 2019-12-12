use std::fs::File;
use std::io::{BufRead, BufReader};

struct Asteroid {
    x: f64,
    y: f64,
}

impl Asteroid {
    fn new(x: f64, y: f64) -> Self {
        Asteroid { x, y }
    }

    fn slope(&self, other: Self) -> f64 {
        let dy = (other.y - self.y);
        let dx = (other.x - self.x);
        dy.atan2(dx)
    }
}

struct AsteroidMap {
    asteroids: Vec<Asteroid>,
}

impl AsteroidMap {
    //    fn new(raw_map: &str) -> Self {}

    // calculate slopes from each asteroid to every other and count unique.
    fn best_location(&self) -> Asteroid {
        Asteroid::new(0.0, 0.0)
    }
}
//
//// FROM DAY 1
//fn read_input_file(path: &str) -> Vec<String> {
//    let file = File::open(path).unwrap();
//    let reader = BufReader::new(file);
//
//    let mut inputs = vec![];
//    for line in reader.lines() {
//        inputs.push(line.unwrap());
//    }
//
//    inputs
//}

fn do_part1() {
    println!("Part 1 answer: {}", 42);
}

fn do_part2() {
    println!("Part 2 answer: {}", 42);
}

fn main() {
    println!("Hello, world!");
}

// TODO TESTS:

/*
.#..#
.....
#####
....#
...##
(3,4); 8

......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####
(5,8); 33

#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.
(1,2); 35

.#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..
(6,3); 41


.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
(11,13); 210

*/
