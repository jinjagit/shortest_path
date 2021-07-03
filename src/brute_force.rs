#[path = "utils.rs"]
mod utils;

use itertools::Itertools;

pub fn brute_unoptimized(coords: Vec<(f32, f32)>) -> (Vec<(f32, f32)>, f32, u32) {
    // Define coords to use for line series & points
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
            total_d += utils::distance(coords[*path[i]], coords[*path[i + 1]]);
        }

        total_d += utils::distance(coords[*path[path.len() - 1]], coords[0]);

        println!("{:?} = {:?}", path, total_d);

        if total_d < shortest {
            shortest = total_d;
            best_path = path.clone();
        }

        count += 1;
    }

    return (utils::reorder_coords(coords, best_path), shortest, count);
}
