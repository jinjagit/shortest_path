#[path = "utils.rs"]
mod utils;

use itertools::Itertools;

pub fn brute_force(coords: Vec<(f32, f32)>) -> (Vec<(f32, f32)>, f32, u32) {
    let n = coords.len(); // Number of points provided
    let indices: Vec<usize> = utils::create_indices_vec(n);
    let mut count: u32 = 0;
    let mut best_path: Vec<&usize> = vec![];
    let mut shortest: f32 = 999999.9;

    // Create matrix of distances between points.
    let matrix: Vec<Vec<f32>> = utils::distance_matrix(coords.clone(), n);

    // iterate over permutations of indices 1..n
    for perm in indices.iter().permutations(indices.len()).unique() {
        let mut p = perm.clone();
        let mut path: Vec<&usize> = vec![&0];
        path.append(&mut p);

        let mut total_d: f32 = 0.0;

        for i in 0..path.len() - 1 {
            total_d += matrix[*path[i]][*path[i + 1]];
        }

        total_d += matrix[*path[path.len() - 1]][0];

        // println!("{:?} = {:?}", path, total_d);

        if total_d < shortest {
            shortest = total_d;
            best_path = path.clone();
        }

        count += 1;
    }

    return (utils::reorder_coords(coords, best_path), shortest, count);
}
