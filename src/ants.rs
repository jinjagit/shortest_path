// Based on these Java snippets, by Baeldung: https://www.baeldung.com/java-ant-colony-optimization,
// with some small implementation changes + accommodations of differences between Rust & Java.

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

pub fn ant_force(coords: Vec<(f32, f32)>) -> (Vec<usize>, f32, Vec<(Vec<Vec<f32>>, usize)>) {
    let n: usize = coords.len(); // Number of points (cities)
    let mut ants: Vec<Ant> = vec![];
    let iterations: usize = 1024; // Number of simulation iterations to run
    let snapshots: Vec<usize> = vec![2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]; // Iterations to plot

    let c: f32 = 1.0; // The original value of all pheromone trails, at the start of the simulation
    let alpha: f32 = 1.0; // Controls the pheromone importance
    let beta: f32 = 5.0; // Controls the distance priority. Should, generally, be > alpha.
    let evaporation: f32 = 0.5; // The percent of pheromone evaporating every iteration
    let q: f32 = 500.0; // Info. about the total amount of pheromone left on the trail by each Ant
    let ant_factor: f32 = 0.8; // How many ants we'll use per city
    let random_factor: f32 = 0.01; // Chance each ant will simply randomly choose next city to visit

    // Create matrix of distances between cities.
    let distance_matrix: Vec<Vec<f32>> = utils::distance_matrix(coords.clone(), n);
    // Create matrix of visibility of other cities from all cities, using distance_matrix.
    let visibility_matrix: Vec<Vec<f32>> = visibility_matrix(distance_matrix.clone());
    // Create matrix of pheromone trails between cities, all set to initial value of c.
    let mut trails_matrix: Vec<Vec<f32>> = vec![vec![c; n]; n];

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

    let mut best_route: Vec<usize> = vec![];
    let mut best_route_length: f32 = 0.0;

    // Record of trails_matrices, for use in plotting png files for animation
    let mut trails_record: Vec<(Vec<Vec<f32>>, usize)> = vec![];

    // ========== iteration start ==================================================================

    for i in 0..iterations {
        // Move ants: Each ant 'finds' a path, starting at random city, that visits each city once only.
        ants = move_ants(
            ants.clone(),
            n,
            n_ants,
            random_factor,
            trails_matrix.clone(),
            visibility_matrix.clone(),
            alpha,
            beta,
        );

        // Update pheromone trails;
        trails_matrix = update_trails(
            ants.clone(),
            n_ants,
            n,
            evaporation,
            q,
            distance_matrix.clone(),
            trails_matrix.clone(),
        );

        // Update best route found so far (and its length)
        let (updated_best_route, updated_best_route_length) = update_best(
            ants.clone(),
            best_route.clone(),
            best_route_length,
            distance_matrix.clone(),
        );
        best_route = updated_best_route;
        best_route_length = updated_best_route_length;

        // Reset ants if n iterations not yet done:
        if i < iterations - 1 {
            for j in 0..n_ants {
                ants[j].reset(n, indices.clone());

                ants[j].visit(rng.gen_range(0..n));
            }
        }

        // if (i + 1) % 50 == 0 {
        //     trails_record.push((trails_matrix.clone(), i + 1));
        // }

        for j in 0..snapshots.len() {
            if (i + 1) == snapshots[j] {
                trails_record.push((trails_matrix.clone(), i + 1));
            }
        }
    }

    // ========== iteration end ==================================================================

    // println!("best_route_length: {:?}", best_route_length);
    // println!("best_route: {:?}", best_route);

    // let result: String = format!("{:.6}", best_route_length);

    // if result == "2.884553" {
    //     println!("CORRECT")
    // } else {
    //     println!(
    //         "INCORRECT: {:?}% of shortest path",
    //         (best_route_length / 2.8845527) * 100.0
    //     );
    // }

    // for a in ants {
    //     println!("{:?}", a);
    // }

    // for t in trails_matrix {
    //     println!("{:?}", t);
    // }

    (best_route, best_route_length, trails_record)
}

// Create matrix of visibility of other points from all points, using distances in distance_matrix.
// Will produce values of 'inf' for distance between point & self, as expected.
// 'inf' values can be found using if value.is_finite() == false
fn visibility_matrix(distance_matrix: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let mut visibility_matrix: Vec<Vec<f32>> = vec![];

    for i in 0..distance_matrix.len() {
        visibility_matrix.push(
            distance_matrix[i]
                .clone()
                .into_iter()
                .map(|x| 1.0 / x)
                .collect(),
        );
    }

    visibility_matrix
}

// Return vec indices, starting at 0, and ending at n - 1.
fn create_indices_vec(n: usize) -> Vec<usize> {
    let mut indices: Vec<usize> = vec![];

    for i in 0..n {
        indices.push(i);
    }

    indices
}

// Each ant 'finds' a path, starting at random city, that visits each city once only.
// Return vec of ants (which contains path found by each ant)
fn move_ants(
    mut ants: Vec<Ant>,
    n: usize,
    n_ants: usize,
    random_factor: f32,
    trails_matrix: Vec<Vec<f32>>,
    visibility_matrix: Vec<Vec<f32>>,
    alpha: f32,
    beta: f32,
) -> Vec<Ant> {
    let mut rng = rand::thread_rng();
    let mut probabilities: Vec<f32> = vec![0.0; n];

    // n - 1 loops, as we have already selected start city, and will end at start
    for i in 0..n - 1 {
        for j in 0..n_ants {
            // Select city to visit next

            // Decide if should just randomly choose
            let r: f32 = rng.gen();
            let ant = &mut ants[j];

            if r < random_factor {
                // Randomly select a city not yet visited
                ant.visit(ant.not_visited[rng.gen_range(0..n - 1 - i)]);
            } else {
                // Select city using distance & pheromone weightings

                // public void calculateProbabilities(Ant ant) {

                // Calulate total pheromone value, based on cities not yet visited
                let cur_loc: usize = ant.route[ant.route.len() - 1];
                let mut pheromone: f32 = 0.0;

                for i in 0..n {
                    if ant.visited[i] == false {
                        pheromone += trails_matrix[cur_loc][i].powf(alpha)
                            * visibility_matrix[cur_loc][i].powf(beta);
                    }
                }

                // Use pheromone value to caclulate probablities
                for i in 0..n {
                    if ant.visited[i] == true {
                        probabilities[i] = 0.0;
                    } else {
                        let numerator: f32 = trails_matrix[cur_loc][i].powf(alpha)
                            * visibility_matrix[cur_loc][i].powf(beta);

                        if numerator == 0.0 && pheromone == 0.0 {
                            // Handle special case where probability would be NaN
                            probabilities[i] = 1.0;
                        } else {
                            probabilities[i] = numerator / pheromone;
                        }
                    }
                }

                // println!("probs: {:?}", probabilities);

                // Use probablities vec to decide which city to visit next
                let rand: f32 = rng.gen();
                let mut total: f32 = 0.0;

                let mut ok: bool = false;

                for i in 0..n {
                    total += probabilities[i];

                    if total >= rand {
                        ant.visit(i);

                        ok = true;

                        break; // Return here, if convert to fn
                    }
                }

                if ok == false {
                    println!("NO city visited!!!");
                    println!("probs: {:?}", probabilities);
                }
            }
        }

        // print!("\n");
    }

    ants
}

// Update matrix of pheromone trails after each iteration of simulation
fn update_trails(
    ants: Vec<Ant>,
    n_ants: usize,
    n: usize,
    evaporation: f32,
    q: f32,
    distance_matrix: Vec<Vec<f32>>,
    mut trails_matrix: Vec<Vec<f32>>,
) -> Vec<Vec<f32>> {
    // Evaporate (reduce) all pheromone trails by evaporation factor
    for i in 0..n {
        for j in 0..n {
            trails_matrix[i][j] *= evaporation;
        }
    }

    // Strengthen pheromone trails relative to each ant's route, weighted by each route's distance
    // Shorter total route distance increases the pheromone contributions from the respective ant.
    for i in 0..n_ants {
        let contribution: f32 = q / ants[i].route_length(distance_matrix.clone());

        for j in 0..n - 1 {
            let start: usize = ants[i].route[j];
            let end: usize = ants[i].route[j + 1];

            trails_matrix[start][end] += contribution;
            trails_matrix[end][start] += contribution;
        }

        let start: usize = ants[i].route[n - 1];
        let end: usize = ants[i].route[0];

        trails_matrix[start][end] += contribution;
        trails_matrix[end][start] += contribution;
    }

    trails_matrix
}

fn update_best(
    ants: Vec<Ant>,
    mut best_route: Vec<usize>,
    mut best_route_length: f32,
    distance_matrix: Vec<Vec<f32>>,
) -> (Vec<usize>, f32) {
    if best_route == [] {
        best_route = ants[0].route.clone();
        best_route_length = ants[0].route_length(distance_matrix.clone());
    }

    for i in 0..ants.len() {
        let route_length: f32 = ants[i].route_length(distance_matrix.clone());

        if route_length < best_route_length {
            best_route = ants[i].route.clone();
            best_route_length = route_length;
        }
    }

    (best_route, best_route_length)
}
