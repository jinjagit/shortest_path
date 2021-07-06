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

    fn has_visited(&self, city: usize) -> bool {
        return self.visited[city];
    }

    fn route_length(&self, distance_matrix: Vec<Vec<f32>>) -> f32 {
        let mut distance: f32 = distance_matrix[self.route[self.route.len() - 1]][self.route[0]];

        for i in 0..self.route.len() - 1 {
            distance += distance_matrix[self.route[i]][self.route[i + 1]];
        }

        distance
    }

    fn reset(&mut self, n: usize) {
        self.route = vec![];
        self.visited = vec![false; n];
    }
}

pub fn ant_force(coords: Vec<(f32, f32)>) {
    let n: usize = coords.len(); // Number of points (cities)
    let mut ants: Vec<Ant> = vec![];

    let c: f32 = 1.0; // The original value of all pheromone trails, at the start of the simulation
    let alpha: f32 = 1.0; // Controls the pheromone importance
    let beta: f32 = 5.0; // Controls the distance priority. Should, generally, be > alpha.
    let evaporation: f32 = 0.5; // The percent of pheromone evaporating every iteration
    let q: f32 = 500.0; // Info. about the total amount of pheromone left on the trail by each Ant
    let ant_factor: f32 = 0.8; // How many ants we'll use per city
    let random_factor: f32 = 0.50; // Chance each ant will simply randomly choose next city to visit

    // Create matrix of distances between cities.
    let distance_matrix: Vec<Vec<f32>> = utils::distance_matrix(coords.clone(), n);
    // Create matrix of visibility of other cities from all cities, using distance_matrix.
    let visibility_matrix: Vec<Vec<f32>> = visibility_matrix(distance_matrix.clone());
    // Create matrix of pheromone trails between cities, all set to initial value of c.
    let mut trails_matrix: Vec<Vec<f32>> = vec![vec![c; n]; n];

    // let mut probabilities: Vec<f32> = vec![0.0; n]; // TODO: Not sure this is needed here, as in fn

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

    // ========== iteration start ==================================================================

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

    // updateTrails();

    // Update best route found so far (and its length)
    let (updated_best_route, updated_best_route_length) = update_best(
        ants.clone(),
        best_route.clone(),
        best_route_length,
        distance_matrix.clone(),
    );
    best_route = updated_best_route;
    best_route_length = updated_best_route_length;

    println!("best_route_length: {:?}", best_route_length);
    println!("best_route: {:?}", best_route);

    // break if n iterations done, else reset ants:

    // ========== iteration end ==================================================================

    // ants[0].reset(n);

    // DEBUG
    // ants[0].visit(2);
    // ants[0].visit(4);
    // ants[0].visit(1);
    // ants[1].visit(0);

    // println!("Ant 0 has visited 3? {:?}", ants[0].has_visited(3));
    // println!("Ant 0 has visited 4? {:?}", ants[0].has_visited(4));

    for a in ants {
        println!("{:?}", a);
    }
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

    for i in 0..n - 1 {
        // n - 1 loops, as we have already selected start city, and will end at start

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

                        probabilities[i] = numerator / pheromone;
                    }
                }

                // Use probablities vec to decide which city to visit next
                let rand: f32 = rng.gen();
                let mut total: f32 = 0.0;

                for i in 0..n {
                    total += probabilities[i];

                    if total >= rand {
                        ant.visit(i);
                        break; // Return here, if convert to fn
                    }
                }
            }
        }

        // print!("\n");
    }

    ants
}

fn update_best(
    ants: Vec<Ant>,
    mut best_route: Vec<usize>,
    mut best_route_length: f32,
    distance_matrix: Vec<Vec<f32>>,
) -> (Vec<usize>, f32) {
    // Pass in: ants.clone(), best_route, best_route_length, distance_matrix.clone()
    // Return: (best_route, best_route_length)

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
