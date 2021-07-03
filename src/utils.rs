// Find distance between 2 points, using Pythagoras Theorum: c = sqrt(a^2 + b^2)
pub fn distance(point_a: (f32, f32), point_b: (f32, f32)) -> f32 {
  let (x1, y1) = point_a;
  let (x2, y2) = point_b;
  let dx = x1 - x2;
  let dy = y1 - y2;

  return ((dx * dx) + (dy * dy)).sqrt();
}