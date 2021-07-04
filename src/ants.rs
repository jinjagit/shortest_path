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

    // Create matrix of distances between points.
    let distance_matrix: Vec<Vec<f32>> = utils::distance_matrix(coords.clone(), n);

    // DEBUG
    for m in distance_matrix {
      println!("{:?}", m);
    }

    let mut ant1 = Ant {
        route: vec![],
        visited: vec![false; n],
    };

    // DEBUG
    println!("route: {:?}", ant1.route);
    println!("visited: {:?}", ant1.visited);

    ant1.visit(2);
    ant1.visit(4);
    ant1.visit(1);

    println!("route: {:?}", ant1.route);
    println!("visited: {:?}", ant1.visited);
    println!("has visited 3? {:?}", ant1.has_visited(3));
    println!("has visited 4? {:?}", ant1.has_visited(4));
}
