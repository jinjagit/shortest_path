mod plot;

fn main() {
    // Define coords to use for line series & points
    let coords = vec![(0.3, 0.6), (0.7, 0.4), (0.8, 0.7), (0.1, 0.3)];

    // Pass coords to plot function
    plot::plot(coords).unwrap();
}
