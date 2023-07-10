use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{
    shapes::{CompoundShape, Triangle},
    Shape, Vect,
};

pub fn load_obj(filename: &str) -> HashMap<String, Box<dyn Shape>> {
    let mut objects: HashMap<String, Box<dyn Shape>> = HashMap::new();

    let mut vertices = Vec::new();
    let mut shape = CompoundShape::new();

    for line in BufReader::new(File::open(filename).unwrap()).lines() {
        let line = line.unwrap();

        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }

        if tokens[0] == "v" {
            // Vertex
            let v = Vect::new(
                tokens[1].parse().unwrap(),
                tokens[2].parse().unwrap(),
                tokens[3].parse().unwrap(),
            );
            vertices.push(v);
        } else if tokens[0] == "f" {
            fn get_index(face: &str) -> usize {
                face.split('/').next().unwrap().parse::<usize>().unwrap() - 1
            }

            let triangle = Triangle::new(
                vertices[get_index(tokens[1])],
                vertices[get_index(tokens[2])],
                vertices[get_index(tokens[3])],
            );

            shape.add(triangle);
        }
    }

    objects.insert("Shape".into(), shape);

    objects
}
