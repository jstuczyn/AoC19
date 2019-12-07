use std::fs::File;
use std::io::{BufRead, BufReader};

// transitivity

struct OrbitalMap {
    center_of_mass: Orbit,
}

#[derive(Debug)]
struct Orbit {
    center_of_mass_name: String,
    orbiting_objects: Vec<Orbit>,
}

impl Orbit {
    fn new(name: String) -> Self {
        Orbit {
            center_of_mass_name: name,
            orbiting_objects: vec![],
        }
    }

    fn add_orbiting_object(&mut self, orbiting_object: Orbit) {
        self.orbiting_objects.push(orbiting_object);
    }

    fn combine(&mut self, other: Orbit) {
        // make sure we are actually trying to combine right objects
        assert_eq!(self.center_of_mass_name, other.center_of_mass_name);

        self.orbiting_objects
            .extend(other.orbiting_objects.into_iter());
    }
}

fn parse_orbits(raw_orbits: Vec<String>) -> Vec<Orbit> {
    raw_orbits
        .into_iter()
        .map(|raw_orbit| {
            let object_names: Vec<_> = raw_orbit.split(')').collect();
            assert_eq!(2, object_names.len());
            let mut main_orbit = Orbit::new(String::from(object_names[0]));
            let orbiting_object = Orbit::new(String::from(object_names[1]));
            main_orbit.add_orbiting_object(orbiting_object);

            main_orbit
        })
        .collect()
}

fn read_input_file(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut inputs = vec![];
    for line in reader.lines() {
        inputs.push(line.unwrap());
    }

    inputs
}

fn do_part1() {}

fn main() {
    let raw_da6_input = read_input_file("day6.input");
    let raw_orbits = parse_orbits(raw_da6_input);
    println!("{:?}", raw_orbits);
}

/*
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L

42
*/

// AAA)BBB => 1
