use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{
    shapes::{CompoundShape, Triangle},
    Shape, Vect,
};

pub fn load_obj(filename: &str) -> Vec<(String, Box<dyn Shape>)> {
    let mut objects: Vec<(String, Box<dyn Shape>)> = Vec::new();

    let mut vertices = Vec::new();

    let mut name = String::new();
    let mut shape: Option<Box<CompoundShape>> = None;

    for line in BufReader::new(File::open(filename).unwrap()).lines() {
        let line = line.unwrap();

        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }

        if tokens[0] == "o" {
            if let Some(shape) = shape.take() {
                objects.push((name, shape));
            }
            shape = Some(CompoundShape::new());

            name = tokens[1].into();
        } else if tokens[0] == "v" {
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

            shape.as_mut().unwrap().add(triangle);
        }
    }

    objects.push((name, shape.unwrap()));

    objects
}
