use std::borrow::Borrow;
use std::fs;

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
        // firstly assert it has correct length
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
}

// for each height there's a vec of width data
struct Layer(Vec<Vec<usize>>);

impl Layer {
    fn new(layer_data: Vec<usize>, height: usize, width: usize) -> Self {
        let heights: Vec<Vec<usize>> = layer_data.chunks(width).map(|w| w.to_vec()).collect();
        assert_eq!(height, heights.len());
        Layer(heights)
    }

    fn digit_count(&self, digit: usize) -> usize {
        assert!(digit <= 9);
        self.0
            .iter()
            .flat_map(|height| height.iter())
            .filter(|&&d| d == digit)
            .count()
    }
}

fn read_input_file(path: &str) -> Vec<usize> {
    fs::read_to_string(path)
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

fn do_part1(image: Image) {
    let layer_id = image.layer_id_with_fewest_zeroes();
    let layer = image.layer_by_id(layer_id);
    let x = layer.digit_count(1) * layer.digit_count(2);

    println!("Part 1 answer: {}", x);
}

fn main() {
    let mut image = Image::new(25, 6);

    let input = read_input_file("day8.input");
    let layer_chunks = input.chunks(image.area());
    for layer_data in layer_chunks {
        image.add_layer(layer_data.to_vec());
    }

    do_part1(image);
}
