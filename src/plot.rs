use plotters::prelude::*;

pub fn plot(
    line_coords: std::vec::Vec<(f32, f32)>,
    point_coords: std::vec::Vec<(f32, f32)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("images/5.png", (640, 640)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let root = root.margin(10, 10, 10, 20);
    // After this point, we should be able to draw construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("Points & lines :)", ("sans-serif", 30).into_font())
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(0f32..1f32, 0f32..1f32)?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        // We can also change the format of the label text
        // .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    // And we can draw something in the drawing area
    chart.draw_series(LineSeries::new(line_coords, &RED))?;
    // Similarly, we can draw point series
    chart.draw_series(PointSeries::of_element(
        point_coords,
        5, // Size of point
        &RED,
        &|c, s, st| {
            return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
            + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font());
        },
    ))?;
    Ok(())
}
