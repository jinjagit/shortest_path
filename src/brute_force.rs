#[path = "utils.rs"]
mod utils;

use itertools::Itertools;

#[allow(dead_code)]
pub fn brute_unoptimized(coords: Vec<(f32, f32)>) -> (Vec<(f32, f32)>, f32, u32) {
    let n = coords.len(); // Number of points provided
    let indices: Vec<usize> = utils::create_indices_vec(n);

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

#[allow(dead_code)]
pub fn brute_no_duplicates(coords: Vec<(f32, f32)>) -> (Vec<(f32, f32)>, f32, u32) {
    let n = coords.len(); // Number of points provided
    let indices: Vec<usize> = utils::create_indices_vec(n);

    let mut count: u32 = 0;
    let mut best_path: Vec<&usize> = vec![];
    let mut shortest: f32 = 999999.9;

    // Create all permutations of indices 1..n
    let perms: Vec<Vec<&usize>> = indices
        .iter()
        .permutations(indices.len())
        .unique()
        .collect();

    // Remove duplicate routes (inverse ordering of a route == effective duplicate of route)
    // One way to do this is to only accept permutations where first value < last value,
    // in the case of ordered sequential integers
    // perms.retain(|x| x[0] < x[n - 2]);

    for perm in perms {
        if perm[0] < perm[n - 2] {
            let mut p = perm.clone();
            let mut path: Vec<&usize> = vec![&0];
            path.append(&mut p);

            let mut total_d: f32 = 0.0;

            for i in 0..path.len() - 1 {
                total_d += utils::distance(coords[*path[i]], coords[*path[i + 1]]);
            }

            total_d += utils::distance(coords[*path[path.len() - 1]], coords[0]);

            if total_d < shortest {
                shortest = total_d;
                best_path = path.clone();
            }

            count += 1;

            println!("{:?} = {:?}", path, total_d);
        }
    }

    return (utils::reorder_coords(coords, best_path), shortest, count);
}

pub fn brute_matrix(coords: Vec<(f32, f32)>) -> (Vec<(f32, f32)>, f32, u32) {
    let n = coords.len(); // Number of points provided
    let indices: Vec<usize> = utils::create_indices_vec(n);

    let mut count: u32 = 0;
    let mut best_path: Vec<&usize> = vec![];
    let mut shortest: f32 = 999999.9;

    // Create matrix of distances between points. We can use fixed-length arrays.
    let mut matrix: Vec<Vec<f32>> = vec![vec![0.0; n]; n];

    for i in 0..n {
        for j in i + 1..n {
            let d: f32 = utils::distance(coords[i], coords[j]);
            matrix[i][j] = d;
            matrix[j][i] = d;
        }
    }

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

        println!("{:?} = {:?}", path, total_d);

        if total_d < shortest {
            shortest = total_d;
            best_path = path.clone();
        }

        count += 1;
    }

    return (utils::reorder_coords(coords, best_path), shortest, count);
}
