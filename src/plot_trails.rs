use plotters::prelude::*;

#[allow(dead_code)]
pub fn plot_trails(
    coords: std::vec::Vec<(f32, f32)>,
    title: &str,
    file_path: &str,
    matrix: Vec<Vec<f32>>,
    best_route: Vec<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_path: &str = &(format!("{}", file_path));
    let root = BitMapBackend::new(file_path, (640, 640)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let root = root.margin(10, 10, 10, 20);
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 24).into_font())
        .x_label_area_size(20)
        .y_label_area_size(40)
        .build_cartesian_2d(0f32..1f32, 0f32..1f32)?;

    // Draw pheromone trails
    let n = coords.len();

    for i in 0..n {
        for j in i + 1..n {
            let mut line_coords: Vec<(f32, f32)> = vec![];
            line_coords.push(coords[i]);
            line_coords.push(coords[j]);

            let mut weight: f64 = matrix[i][j] as f64;
            // if matrix[i][j] < 0.75 {
            //     weight = matrix[i][j] as f64 * 3.0;
            // }

            chart.draw_series(LineSeries::new(line_coords, &BLUE.mix(weight)))?;
        }
    }

    if best_route != [] {
        // Draw line series
        let mut best_route_coords: Vec<(f32, f32)> = vec![];
        for i in 0..best_route.len() {
            best_route_coords.push(coords[best_route[i]]);
        }

        best_route_coords.push(coords[best_route[0]]);

        // mix == weight, 10.0 max, 0.3 min viable weight
        chart.draw_series(LineSeries::new(best_route_coords, &GREEN.mix(10.0)))?;
    }

    // Draw point series
    chart.draw_series(PointSeries::of_element(
        coords,
        3, // Size of point
        &BLACK,
        &|c, s, st| return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
    ))?;

    Ok(())
}
