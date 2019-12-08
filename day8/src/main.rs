use core::fmt;
use itertools::Itertools;
use std::borrow::Borrow;
use std::fmt::{Display, Error, Formatter};
use std::fs;

const TRANSPARENT_PIXEL: usize = 2;

#[derive(Clone)]
struct Image {
    height: usize,
    width: usize,
    layers: Vec<Layer>,
}

impl Image {
    fn new(height: usize, width: usize) -> Self {
        Image {
            height,
            width,
            layers: Vec::new(),
        }
    }

    fn area(&self) -> usize {
        self.height * self.width
    }

    fn add_layer(&mut self, layer_data: Vec<usize>) {
        assert_eq!(self.area(), layer_data.len());
        let layer = Layer::new(layer_data, self.height, self.width);
        self.layers.push(layer);
    }

    fn layer_id_with_fewest_zeroes(&self) -> usize {
        let (i, _) = self
            .layers
            .iter()
            .enumerate()
            .map(|(i, layer)| (i, layer.digit_count(0)))
            .min_by(|(_, c1), (_, c2)| c1.cmp(c2))
            .unwrap();

        i
    }

    fn layer_by_id(&self, id: usize) -> &Layer {
        self.layers[id].borrow()
    }

    fn cover_all_layers(self) -> Layer {
        self.layers
            .into_iter()
            .rev()
            .fold1(|bottom, top| bottom.cover(&top))
            .unwrap()
    }
}

// for each height there's a vec of width data
#[derive(Debug, PartialEq, Clone)]
struct Layer(Vec<Vec<usize>>);

impl Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::char;

        for width in self.0.iter() {
            let width_as_chars = width
                .iter()
                .map(|&d| char::from_digit(d as u32, 10).unwrap())
                .collect::<String>();
            write!(f, "{}\n", width_as_chars)?;
        }
        Ok(())
    }
}

impl Layer {
    fn new(layer_data: Vec<usize>, height: usize, width: usize) -> Self {
        let widths: Vec<Vec<usize>> = layer_data.chunks(width).map(|w| w.to_vec()).collect();
        assert_eq!(height, widths.len());
        Layer(widths)
    }

    fn transparent(height: usize, width: usize) -> Self {
        Layer::new(
            std::iter::repeat(TRANSPARENT_PIXEL)
                .take(height * width)
                .collect(),
            height,
            width,
        )
    }

    fn digit_count(&self, digit: usize) -> usize {
        assert!(digit <= 9);
        self.0
            .iter()
            .flat_map(|width| width.iter())
            .filter(|&&d| d == digit)
            .count()
    }

    fn cover(&self, top: &Layer) -> Self {
        let resultant_layer_data: Vec<_> = self
            .0
            .iter()
            .flat_map(|width| width.iter())
            .zip(top.0.iter().flat_map(|width| width.iter()))
            .map(|(&orig, &cover)| {
                if cover == TRANSPARENT_PIXEL {
                    orig
                } else {
                    cover
                }
            })
            .collect();

        Layer::new(resultant_layer_data, self.0.len(), self.0[0].len())
    }
}

fn read_input_file(path: &str) -> Vec<usize> {
    fs::read_to_string(path)
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

fn do_part2(image: Image) {
    let visible_data = image.cover_all_layers();
    println!("Part 2 answer: \n{}", visible_data);
}

fn do_part1(image: Image) {
    let layer_id = image.layer_id_with_fewest_zeroes();
    let layer = image.layer_by_id(layer_id);
    let x = layer.digit_count(1) * layer.digit_count(2);

    println!("Part 1 answer: {}", x);
}

fn main() {
    let mut image = Image::new(6, 25);

    let input = read_input_file("day8.input");
    let layer_chunks = input.chunks(image.area());

    for layer_data in layer_chunks {
        image.add_layer(layer_data.to_vec());
    }

    do_part1(image.clone());
    do_part2(image);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn covering_layer_with_transparent_layer_doesnt_change_the_layer() {
        let height = 3;
        let width = 3;

        let layer = Layer::new(vec![0, 1, 2, 2, 1, 0, 0, 1, 0], height, width);
        let transparent = Layer::transparent(height, width);

        assert_eq!(layer, layer.cover(&transparent))
    }
}
