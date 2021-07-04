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
}

pub fn ant_force(coords: Vec<(f32, f32)>) {
    let n = coords.len();
    let mut ants: Vec<Ant> = vec![];

    // Create matrix of distances between points.
    // let distance_matrix: Vec<Vec<f32>> = utils::distance_matrix(coords.clone(), n);

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
