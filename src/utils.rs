// Find distance between 2 points, using Pythagoras Theorum: c = sqrt(a^2 + b^2)
pub fn distance(point_a: (f32, f32), point_b: (f32, f32)) -> f32 {
  let (x1, y1) = point_a;
  let (x2, y2) = point_b;
  let dx = x1 - x2;
  let dy = y1 - y2;

  return ((dx * dx) + (dy * dy)).sqrt();
}

// Return vec of coords reordered using vec of indexes representing shortest_path through coords
pub fn reorder_coords(coords: Vec<(f32, f32)>, best_path: Vec<&usize>) -> Vec<(f32, f32)> {
  let mut best_path_coords: Vec<(f32, f32)> = vec![];

    for e in best_path {
        best_path_coords.push(coords[*e]);
    }

    best_path_coords
}
