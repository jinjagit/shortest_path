#[path = "utils.rs"]
mod utils;

#[derive(Debug)] // This annotation allows debugging print of struct
struct Ant {
    route: Vec<usize>,
    visited: Vec<bool>,
}

impl Ant {
    fn visit(&mut self, city: usize) {
        self.route.push(city);
        self.visited[city] = true;
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
}

pub fn ant_force(coords: Vec<(f32, f32)>) {
    let n = coords.len(); // Number of points (cities)
    let mut ants: Vec<Ant> = vec![];

    let c: f32 = 1.0; // The original number of trails, at the start of the simulation
    let alpha: f32 = 1.0; // Controls the pheromone importance
    let beta: f32 = 5.0; // Controls the distance priority. Should, generally, be > alpha.
    let evaporation: f32 = 0.5; // The percent of pheromone evaporating every iteration
    let Q: f32 = 500.0; // Info. about the total amount of pheromone left on the trail by each Ant
    let n_ants: f32 = 0.8; // How many ants we'll use per city
    let random_factor: f32 = 0.01;

    // Create matrix of distances between points.
    let distance_matrix: Vec<Vec<f32>> = utils::distance_matrix(coords.clone(), n);
    // Create matrix of visibility of other points from all points, using distance_matrix.
    let visibility_matrix: Vec<Vec<f32>> = utils::visibility_matrix(distance_matrix.clone());

    let n_ants: usize = 3; // Set number of ants

    // Add ants, each in default state, to 'ants' vec
    for _ in 0..n_ants {
        ants.push(Ant {
            route: vec![],
            visited: vec![false; n],
        });
    }

    // DEBUG
    ants[0].visit(2);
    ants[0].visit(4);
    ants[0].visit(1);
    ants[1].visit(0);

    println!("Ant 0 has visited 3? {:?}", ants[0].has_visited(3));
    println!("Ant 0 has visited 4? {:?}", ants[0].has_visited(4));

    for ant in ants {
        println!("{:?}", ant);
    }
}
