mod plot;

fn main() {
    // define coords to use for line series & points
    let line_coords = vec![(0.0, 0.0), (0.5, 0.5), (0.8, 0.7), (0.0, 0.0)];
    let point_coords = vec![(0.0, 0.0), (0.5, 0.5), (0.8, 0.7), (0.3, 0.5)];

    // pass coords to plot function
    plot::plot(line_coords, point_coords).unwrap();
}
