#[path = "utils.rs"]
mod utils;

use rand::prelude::*;

#[derive(Debug)] // This annotation allows debugging print of struct
#[derive(Clone)]
struct Ant {
    route: Vec<usize>,
    visited: Vec<bool>,
    not_visited: Vec<usize>,
}

impl Ant {
    fn visit(&mut self, city: usize) {
        self.route.push(city);
        self.visited[city] = true;
        self.not_visited.retain(|x| *x != city);
    }

    fn route_length(&self, distance_matrix: Vec<Vec<f32>>) -> f32 {
        let mut distance: f32 = distance_matrix[self.route[self.route.len() - 1]][self.route[0]];

        for i in 0..self.route.len() - 1 {
            distance += distance_matrix[self.route[i]][self.route[i + 1]];
        }

        distance
    }

    fn reset(&mut self, n: usize, indices: Vec<usize>) {
        self.route = vec![];
        self.visited = vec![false; n];
        self.not_visited = indices;
    }
}

pub fn random_walks(coords: Vec<(f32, f32)>) -> Vec<usize> {
    let n: usize = coords.len(); // Number of points (cities)
    let mut ants: Vec<Ant> = vec![];
    let iterations: usize = 210; // Number of simulation iterations to run
    let ant_factor: f32 = 0.8; // How many ants we'll use per city

    // Create matrix of distances between cities.
    let distance_matrix: Vec<Vec<f32>> = utils::distance_matrix(coords.clone(), n);

    // Set number of ants. Always rounds down.
    let n_ants: usize = (n as f32 * ant_factor) as usize;

    let mut rng = rand::thread_rng();
    let indices: Vec<usize> = create_indices_vec(n);

    // Add ants, each in default state, to 'ants' vec.
    for i in 0..n_ants {
        ants.push(Ant {
            route: vec![],
            visited: vec![false; n],
            not_visited: indices.clone(),
        });

        // Set each ant to start at a random city.
        ants[i].visit(rng.gen_range(0..n));
    }

    // let mut best_route: Vec<usize> = vec![];
    let mut best_route_length: f32 = 0.0;

    let mut worst_route: Vec<usize> = vec![];
    let mut worst_route_length: f32 = 0.0;

    let mut all_lengths: Vec<f32> = vec![];

    for i in 0..iterations {
        // n - 1 loops, as we have already selected start city, and will end at start
        for j in 0..n - 1 {
            for k in 0..ants.len() {
                let not_visited = ants[k].not_visited.clone();
                ants[k].visit(not_visited[rng.gen_range(0..n - 1 - j)]);
            }
        }

        for j in 0..ants.len() {
            let length = ants[j].route_length(distance_matrix.clone());
            let route = ants[j].route.clone();

            if j == 0 {
                best_route_length = length;
                worst_route_length = length;
                // best_route = route.clone();
                worst_route = route.clone();
            }

            if length < best_route_length {
                best_route_length = length;
                // best_route = route.clone();
            } else if length > worst_route_length {
                worst_route_length = length;
                worst_route = route.clone();
            }

            all_lengths.push(length);
        }

        // Reset ants if n iterations not yet done:
        if i < iterations - 1 {
            for j in 0..n_ants {
                ants[j].reset(n, indices.clone());

                ants[j].visit(rng.gen_range(0..n));
            }
        }
    }

    let sum: f32 = all_lengths.iter().sum();
    let average_length: f32 = sum / all_lengths.len() as f32;

    println!("------- Random walks ---------");
    println!("n of random routes: {}", n_ants * iterations);
    println!("best route length: {}", best_route_length);
    println!("worst route length: {}", worst_route_length);
    println!("average route length: {}", average_length);

    worst_route
}

// Return vec indices, starting at 0, and ending at n - 1.
fn create_indices_vec(n: usize) -> Vec<usize> {
    let mut indices: Vec<usize> = vec![];

    for i in 0..n {
        indices.push(i);
    }

    indices
}
