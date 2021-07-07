mod ants;
mod brute_force;
mod plot;
mod plot_trails;
mod apng;

use ants::ant_force;
use rand::prelude::*;

fn main() {
    // // Define coords to use for line series & points
    // let n = 10; // Number of points we want
    // let coords: Vec<(f32, f32)> = create_points(n);

    let coords: Vec<(f32, f32)> = vec![
        (0.75, 0.32),
        (0.154, 0.83),
        (0.44, 0.77),
        (0.3456, 0.7654),
        (0.111, 0.222),
        (0.9, 0.876),
        (0.23, 0.6389),
        (0.05, 0.78),
        (0.63, 0.25),
        (0.33, 0.415),
    ];

    // let (best_path_coords, shortest, count): (Vec<(f32, f32)>, f32, u32) =
    //     brute_force::brute_force(coords);

    // println!("permutations (n - 1)! = {:?}", count);
    // println!("shortest = {:?}", shortest);

    // // Create chart of brute-force shortest path
    // plot::plot(best_path_coords, "Brute-force solution", "brute-force-10.png").unwrap();

    let (best_route, best_route_length, mut trails_record): (Vec<usize>, f32, Vec<(Vec<Vec<f32>>, usize)>) = ant_force(coords.clone());

    println!("------------------ ACO -------------------");
    println!("Best route: {:?}", best_route);
    println!("Length = {:?}", best_route_length);

    println!("\nEncoding {} PNG files:", trails_record.len());

    trails_record = normalize_trails_record(trails_record.clone());

    plot_trails_record(coords, trails_record.clone(), best_route.clone());

    println!("success");

    apng::create_apng();
}

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

fn normalize_matrix(mut matrix: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let n: usize = matrix.len();
    let mut max: f32 = 0.0;

    for i in 0..n {
        for j in i + 1..n {
            if matrix[i][j] > max {
                max = matrix[i][j];
            }
        }
    }

    for i in 0..n {
        for j in i + 1..n {
            let norm: f32 = matrix[i][j] / max;
            matrix[i][j] = norm;
            matrix[j][i] = norm;
        }
    }

    matrix
}
fn normalize_trails_record(mut record: Vec<(Vec<Vec<f32>>, usize)>) -> Vec<(Vec<Vec<f32>>, usize)> {
    for i in 0..record.len() {
        let (matrix, iter) = record[i].clone();

        record[i] = (normalize_matrix(matrix.clone()), iter);
    }

    record
}

fn plot_trails_record(coords: Vec<(f32, f32)>, record: Vec<(Vec<Vec<f32>>, usize)>, mut best_route: Vec<usize>) {
    for i in 0..record.len() {
        let (matrix, iter) = record[i].clone();
        let file_path: &str = &(format!("images/series_1/ants_{}_{}.png", coords.len(), i));
        let title: &str = &(format!("ACO - points: {}, iteration: {}", coords.len(), iter));

        if i < record.len() - 1 {
            plot_trails::plot_trails(coords.clone(), title, file_path, matrix.clone(), vec![]).unwrap();
        } else {
            plot_trails::plot_trails(coords.clone(), title, file_path, matrix, best_route.clone()).unwrap();
        }
    }

}