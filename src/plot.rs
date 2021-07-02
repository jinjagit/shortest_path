use plotters::prelude::*;

pub fn plot(point_coords: std::vec::Vec<(f32, f32)>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("images/5.png", (1280, 1280)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let root = root.margin(10, 10, 10, 20);
    let mut chart = ChartBuilder::on(&root)
        .caption("Points & lines :)", ("sans-serif", 30).into_font())
        .x_label_area_size(20)
        .y_label_area_size(40)
        .build_cartesian_2d(0f32..1f32, 0f32..1f32)?;

    // Draw mesh
    chart.configure_mesh().x_labels(5).y_labels(5).draw()?;

    // Draw line series
    let mut line_coords = point_coords.clone();
    line_coords.push(point_coords[0]);

    chart.draw_series(LineSeries::new(line_coords, &RED))?;

    // Draw point series
    chart.draw_series(PointSeries::of_element(
        point_coords,
        5, // Size of point
        &BLACK,
        &|c, s, st| return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
    ))?;

    Ok(())
}
