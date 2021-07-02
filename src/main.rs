mod plot;

use itertools::Itertools;

fn main() {
    // Define coords to use for line series & points
    let coords: Vec<(f32, f32)> = vec![(0.3, 0.6), (0.7, 0.4), (0.8, 0.7), (0.1, 0.3), (0.7, 0.9)];

    // let d: f32 = distance(coords[0], coords[1]);
    // println!("d = {:.3}", d);

    let n = coords.len();

    // Pass coords to plot function
    // plot::plot(coords).unwrap();

    // Let's find all permutations of a vec
    let mut indices: Vec<usize> = vec![];

    // create vec of indices 1..n-1
    for i in 0..n - 1 {
        indices.push(i + 1);
    }

    let mut count: u32 = 0;
    // let mut best_path: Vec<(f32, f32)> = vec![];


    // create all permutations of indices 0 << 1..n-1
    for perm in indices.iter().permutations(indices.len()).unique() {
        let mut p = perm.clone();
        let mut path: Vec<&usize> = vec![&0];
        path.append(&mut p);

        let mut total_d: f32 = 0.0;

        for i in 0..path.len() - 1 {
            total_d += distance(coords[*path[i]], coords[*path[i + 1]]);
        }

        total_d += distance(coords[*path[path.len() - 1]], coords[0]);

        count += 1;

        println!("{:?} = {:?}", path, total_d);
    }

    println!("permutations (n - 1)! = {:?}", count);
}

pub fn distance(point_a: (f32, f32), point_b: (f32, f32)) -> f32 {
    let (x1, y1) = point_a;
    let (x2, y2) = point_b;

    return (((x1 - x2) * (x1 - x2)) + ((y1 - y2) * (y1 - y2))).sqrt();
}
