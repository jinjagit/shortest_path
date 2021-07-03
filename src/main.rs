mod brute_force;
mod plot;

use rand::prelude::*;

fn main() {
    // Define coords to use for line series & points
    let n = 5; // Number of points we want
    let coords: Vec<(f32, f32)> = create_points(n);

    let (best_path_coords, shortest, count): (Vec<(f32, f32)>, f32, u32) =
        brute_force::brute_unoptimized(coords);

    println!("permutations (n - 1)! = {:?}", count);
    println!("shortest = {:?}", shortest);

    // Create chart of shortest path
    plot::plot(best_path_coords).unwrap();
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
