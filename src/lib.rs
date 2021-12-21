
pub fn build_rounded_rectangle(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    border_radius: [f32; 4],
    segments: usize,
) -> Vec<[f32; 2]> {
    let mut points = Vec::new();

    add_rectangle_corner(&mut points, 90.0, 0.0, border_radius[1], x + w - border_radius[1], y + h - border_radius[1], segments);
    add_rectangle_corner(&mut points, -0.0, -90.0, border_radius[2], x + w - border_radius[2], y + border_radius[2], segments);
    add_rectangle_corner(&mut points, -90.0, -180.0, border_radius[3], x + border_radius[3], y + border_radius[3], segments);
    add_rectangle_corner(&mut points, -180.0, -270.0, border_radius[0], x + border_radius[0], y + h - border_radius[0], segments);

    points
}

pub fn add_rectangle_corner(
    points: &mut Vec<[f32; 2]>,
    starting_angle: f32,
    ending_angle: f32,
    radius: f32,
    base_x: f32,
    base_y: f32,
    segments: usize,
) {
    for i in 0..segments {
        let ratio = (i as f32) / ((segments as f32) - 1.0);
        let angle = (1.0 - ratio) * starting_angle + ratio * ending_angle;
        let angle_rad = angle.to_radians();
        points.push([
            base_x + radius * angle_rad.cos(),
            base_y + radius * angle_rad.sin(),
        ]);
    }
}
