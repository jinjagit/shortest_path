use plotters::prelude::*;

#[allow(dead_code)]
pub fn plot(
    best_path: std::vec::Vec<(f32, f32)>, // mut worst_path: std::vec::Vec<(f32, f32)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("images/plot.png", (640, 640)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let root = root.margin(10, 10, 10, 20);
    let mut chart = ChartBuilder::on(&root)
        .caption("Shortest by brute-force", ("sans-serif", 30).into_font())
        .x_label_area_size(20)
        .y_label_area_size(40)
        .build_cartesian_2d(0f32..1f32, 0f32..1f32)?;

    // Draw mesh
    // chart.configure_mesh().x_labels(5).y_labels(5).draw()?;

    // Draw line series
    let mut line_coords = best_path.clone();
    line_coords.push(best_path[0]);

    // mix == weight, 10.0 max, 0.3 min viable weight
    chart.draw_series(LineSeries::new(line_coords, &BLUE.mix(0.3)))?;

    // // Move coords of worst_path 1.0 to the right
    // for i in 0..worst_path.len() {
    //     let (mut x, y) = worst_path[i];
    //     x += 1.0;
    //     worst_path[i] = (x, y);
    // }

    // line_coords = worst_path.clone();
    // line_coords.push(worst_path[0]);

    // chart.draw_series(LineSeries::new(line_coords, &RED))?;

    // Draw point series
    chart.draw_series(PointSeries::of_element(
        best_path,
        3, // Size of point
        &BLACK,
        &|c, s, st| return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
    ))?;

    // chart.draw_series(PointSeries::of_element(
    //     worst_path,
    //     5, // Size of point
    //     &BLACK,
    //     &|c, s, st| return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
    // ))?;

    Ok(())
}
