use criterion::{black_box, criterion_group, criterion_main, Criterion};
use itertools::Itertools;
use rand::prelude::*;
use std::time::Duration;

fn brute_unoptimized(coords: Vec<(f32, f32)>) -> (Vec<(f32, f32)>, f32, u32) {
    let n = coords.len(); // Number of points we want

    // Find all permutations of a vec
    let mut indices: Vec<usize> = vec![];

    // Create vec of indices 1..n-1
    for i in 1..n {
        indices.push(i);
    }

    let mut count: u32 = 0;
    let mut best_path: Vec<&usize> = vec![];
    let mut shortest: f32 = 999999.9;

    // iterate over permutations of indices 1..n
    for perm in indices.iter().permutations(indices.len()).unique() {
        let mut p = perm.clone();
        let mut path: Vec<&usize> = vec![&0];
        path.append(&mut p);

        let mut total_d: f32 = 0.0;

        for i in 0..path.len() - 1 {
            total_d += distance(coords[*path[i]], coords[*path[i + 1]]);
        }

        total_d += distance(coords[*path[path.len() - 1]], coords[0]);

        if total_d < shortest {
            shortest = total_d;
            best_path = path.clone();
        }

        count += 1;
    }

    // Make vecs of coords from best_path & worst_path vecs (of &usize elements)
    let mut best_path_coords: Vec<(f32, f32)> = vec![];

    for e in best_path {
        best_path_coords.push(coords[*e]);
    }

    return (best_path_coords, shortest, count);
}



// Utils:

// Create a random collection of n points, where 0 <= x <= 1, 0 <= y <= 1
pub fn create_points(n: usize) -> Vec<(f32, f32)> {
    let mut coords: Vec<(f32, f32)> = vec![];
    let mut rng = rand::thread_rng();

    for _ in 0..n {
        let x: f32 = rng.gen();
        let y: f32 = rng.gen();
        coords.push((x, y));
    }

    coords
}

// Find distance between 2 points, using Pythagoras Theorum: c = sqrt(a^2 + b^2)
pub fn distance(point_a: (f32, f32), point_b: (f32, f32)) -> f32 {
    let (x1, y1) = point_a;
    let (x2, y2) = point_b;
    let dx = x1 - x2;
    let dy = y1 - y2;

    return ((dx * dx) + (dy * dy)).sqrt();
}


// Run the Criterion benchmark

fn criterion_benchmark(c: &mut Criterion) {
    let n = 10; // Number of points we want
    let coords: Vec<(f32, f32)> = create_points(n);

    c.bench_function("brute-force 10", |b| b.iter(|| brute_unoptimized(black_box(coords.clone()))));
}

fn set_target_time() -> Criterion {
    Criterion::default().measurement_time(Duration::new(15, 0)).sample_size(50)
}

criterion_group! {
    name = benches;
    config = set_target_time();
    targets = criterion_benchmark
}
criterion_main!(benches);
