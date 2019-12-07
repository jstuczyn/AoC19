use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct OrbitalMap {
    global_center_of_mass: Orbit,
}

impl OrbitalMap {
    fn new(raw_orbits: Vec<Orbit>) -> Self {
        let mut orbit_directory = OrbitalMap::combine_raw_orbits(raw_orbits);
        // start with "COM"
        // if it doesn't exist, panic, because it MUST exist
        let mut com = orbit_directory.remove("COM").unwrap();
        com.extract_orbiting_object_details(&mut orbit_directory);

        OrbitalMap {
            global_center_of_mass: com,
        }
    }

    fn combine_raw_orbits(raw_orbits: Vec<Orbit>) -> HashMap<String, Orbit> {
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

        orbit_directory
    }

    fn total_orbit_count(&self) -> usize {
        self.global_center_of_mass.orbit_count(1)
    }

    fn num_transfers(&self, source_name: &str, destination_name: &str) -> usize {
        // get path from root to those, list all the nodes alongside
        // find the common node
        // then dist from common to either
        let source_path = self.depth_first_search(source_name);
        let dest_path = self.depth_first_search(destination_name);

        let intersection = source_path
            .iter()
            .zip(dest_path.iter())
            .position(|(se, de)| se != de)
            .unwrap_or_else(|| source_path.len());

        (source_path.len() - intersection) + (dest_path.len() - intersection)
    }

    // it's more likely that our target will be deep in the tree rather than close to the root
    fn depth_first_search(&self, target_name: &str) -> Vec<&str> {
        self.global_center_of_mass
            .find_dfs(target_name, vec![])
            .unwrap()
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

    fn orbit_count(&self, chain_length: usize) -> usize {
        self.orbiting_objects
            .iter()
            .map(|orb| chain_length + orb.orbit_count(chain_length + 1))
            .sum()
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

    fn find_dfs<'a>(
        &'a self,
        target_name: &str,
        current_path: Vec<&'a str>,
    ) -> Option<Vec<&'a str>> {
        for orb in &self.orbiting_objects {
            let mut new_path = current_path.clone();
            new_path.push(&self.center_of_mass_name);

            if &orb.center_of_mass_name == target_name {
                return Some(new_path);
            } else {
                match orb.find_dfs(target_name, new_path) {
                    None => (),
                    Some(path) => return Some(path),
                }
            }
        }

        None
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

fn do_part1(orbital_map: &OrbitalMap) {
    println!("Part 1 answer: {}", orbital_map.total_orbit_count());
}

fn do_part2(orbital_map: &OrbitalMap) {
    println!("Part 1 answer: {}", orbital_map.num_transfers("YOU", "SAN"));
}

fn main() {
    let raw_day6_input = read_input_file("day6.input");
    let raw_orbits = parse_orbits(raw_day6_input);
    let orbital_map = OrbitalMap::new(raw_orbits);

    do_part1(&orbital_map);
    do_part2(&orbital_map);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_orbital_map_returns_correct_number_of_orbits() {
        let sample_orbits = vec![
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
        let raw_orbits = parse_orbits(sample_orbits);
        let orbital_map = OrbitalMap::new(raw_orbits);

        assert_eq!(42, orbital_map.total_orbit_count());
    }

    #[test]
    fn sample_orbital_map_returns_correct_number_of_orbital_transfers() {
        let sample_orbits = vec![
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
            String::from("K)YOU"),
            String::from("I)SAN"),
        ];
        let raw_orbits = parse_orbits(sample_orbits);
        let orbital_map = OrbitalMap::new(raw_orbits);

        assert_eq!(4, orbital_map.num_transfers("YOU", "SAN"));
    }

    #[test]
    fn sample_2_orbital_map_returns_correct_number_of_orbital_transfers() {
        let sample_orbits = vec![
            String::from("COM)A"),
            String::from("A)B"),
            String::from("B)C"),
            String::from("C)D"),
            String::from("A)YOU"),
            String::from("D)SAN"),
        ];
        let raw_orbits = parse_orbits(sample_orbits);
        let orbital_map = OrbitalMap::new(raw_orbits);

        assert_eq!(3, orbital_map.num_transfers("YOU", "SAN"));
    }
}
