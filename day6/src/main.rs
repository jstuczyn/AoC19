use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

// transitivity

#[derive(Debug)]
struct OrbitalMap {
    global_center_of_mass: Orbit,
}

impl OrbitalMap {
    fn construct(mut orbit_directory: HashMap<String, Orbit>) -> Self {
        // start with "COM"
        // if it doesn't exist, panic, because it MUST exist
        let mut com = orbit_directory.remove("COM").unwrap();

        com.extract_orbiting_object_details(&mut orbit_directory);

        OrbitalMap {
            global_center_of_mass: com,
        }
    }
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

    fn extract_orbiting_object_details(&mut self, orbit_directory: &mut HashMap<String, Orbit>) {
        for mut orbiting_object in &mut self.orbiting_objects {
            match orbit_directory.remove(&orbiting_object.center_of_mass_name) {
                None => {
                    // it's a leaf node so we don't need to do anything
                }
                Some(orbit_details) => {
                    orbiting_object.orbiting_objects = orbit_details.orbiting_objects;
                    orbiting_object.extract_orbiting_object_details(orbit_directory);
                }
            }
        }
    }

    fn combine(&mut self, other: Orbit) {
        // make sure we are actually trying to combine right objects
        assert_eq!(self.center_of_mass_name, other.center_of_mass_name);

        self.orbiting_objects
            .extend(other.orbiting_objects.into_iter());
    }
}

fn combine_orbits(raw_orbits: Vec<Orbit>) {
    let mut orbit_directory: HashMap<String, Orbit> = HashMap::new();

    for orbit in raw_orbits {
        match orbit_directory.get_mut(&orbit.center_of_mass_name) {
            Some(orb) => {
                orb.combine(orbit);
            }
            None => {
                orbit_directory.insert(orbit.center_of_mass_name.clone(), orbit);
            }
        }
    }

    let orbital_map = OrbitalMap::construct(orbit_directory);
    println!("\n\nFinal map: {:?}", orbital_map);
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
    let raw_day6_input = read_input_file("day6.input");

    let raw_day6_input = vec![
        String::from("COM)B"),
        String::from("B)C"),
        String::from("C)D"),
        String::from("D)E"),
        String::from("E)F"),
        String::from("B)G"),
        String::from("G)H"),
        String::from("D)I"),
        String::from("E)J"),
        String::from("J)K"),
        String::from("K)L"),
    ];

    let raw_orbits = parse_orbits(raw_day6_input);
    combine_orbits(raw_orbits);
    //    println!("{:?}", raw_orbits);
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
