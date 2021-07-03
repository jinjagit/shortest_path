mod plot;

use itertools::Itertools;
use rand::prelude::*;

fn main() {
    // Define coords to use for line series & points
    let mut coords: Vec<(f32, f32)> = vec![];

    let mut rng = rand::thread_rng();
    let n = 5; // Number of points we want to use

    for _ in 0..n {
        let x: f32 = rng.gen();
        let y: f32 = rng.gen();
        coords.push((x, y));
    }

    println!("{:?}\n", coords);

    // Find all permutations of a vec
    let mut indices: Vec<usize> = vec![];

    // Create vec of indices 1..n-1
    for i in 1..n {
        indices.push(i);
    }

    let mut count: u32 = 0;
    let mut best_path: Vec<&usize> = vec![];
    let mut worst_path: Vec<&usize> = vec![];
    let mut shortest: f32 = 999999.9;
    let mut longest: f32 = 0.0;

    // Create all permutations of indices 1..n
    // Remove duplicate routes (inverse ordering of a route == effective duplicate of route):
    // Iterate 1/2 length of list times
    // Compare 0 with 1..last, until match (reverse) found, remove match, break
    // Compare 1 with 2.. last, etc.

    // Create all permutations of indices 1..n
    for perm in indices.iter().permutations(indices.len()).unique() {
        let mut p = perm.clone();
        let mut path: Vec<&usize> = vec![&0];
        path.append(&mut p);

        let mut total_d: f32 = 0.0;

        //println!("path: {:?}", path);

        for i in 0..path.len() - 1 {
            total_d += distance(coords[*path[i]], coords[*path[i + 1]]);
        }

        total_d += distance(coords[*path[path.len() - 1]], coords[0]);

        if total_d < shortest {
            shortest = total_d;
            best_path = path.clone();
        } else if total_d > longest {
            longest = total_d;
            worst_path = path.clone();
        }

        count += 1;

        println!("{:?} = {:?}", path, total_d);
    }

    println!("permutations (n - 1)! = {:?}", count);
    println!("shortest = {:?}", shortest);
    println!("longest = {:?}", longest);

    // Make vecs of coords from best_path & worst_path vecs (of &usize elements)
    let mut best_path_coords: Vec<(f32, f32)> = vec![];

    for e in best_path {
        best_path_coords.push(coords[*e]);
    }

    let mut worst_path_coords: Vec<(f32, f32)> = vec![];

    for e in worst_path {
        worst_path_coords.push(coords[*e]);
    }

    // Create chart of best and worst paths (green lines vs red lines)
    plot::plot(best_path_coords, worst_path_coords).unwrap();
}

// Find distance between 2 points, using Pythagoras Theorum: c = sqrt(a^2 + b^2)
pub fn distance(point_a: (f32, f32), point_b: (f32, f32)) -> f32 {
    let (x1, y1) = point_a;
    let (x2, y2) = point_b;
    let dx = x1 - x2;
    let dy = y1 - y2;

    return ((dx * dx) + (dy * dy)).sqrt();
}

// 0, 2

// [(0, 1, 9.9876).....     ]