use macroquad::prelude::*;
use std::f32::consts::PI;

// custom rectangle drawer due to lack of needed feature of standard drawer.
// currently support gradient with couple of color with custom angle
// and rounded corners.
//
//
//
pub fn draw_rectangle_extended(
    x: f32, 
    y: f32, 
    w: f32, 
    h: f32, 
    r: f32,
    color_1: Color, 
    color_2: Color,
    gradient_angle: f32
) {
    let max_r = f32::min(w, h) / 2.0;
    let r = f32::min(r, max_r);

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let center_x = x + w / 2.0;
    let center_y = y + h / 2.0;
    let center_color = Color::new(
        (color_1.r + color_2.r) / 2.0,
        (color_1.g + color_2.g) / 2.0,
        (color_1.b + color_2.b) / 2.0,
        (color_1.a + color_2.a) / 2.0,
    );
    
    vertices.push(Vertex {
        position: vec3(center_x, center_y, 0.0),
        uv: vec2(0.5, 0.5),
        color: center_color.into(),
        normal: vec4(0.0, 0.0, 1.0, 0.0)
    });

    let segments = 12;

    let mut add_corner_arc = |cx: f32, cy: f32, start_angle: f32, end_angle: f32| {
        for i in 0..=segments {
            let angle = start_angle + (end_angle - start_angle) * (i as f32 / segments as f32);
            let vx = cx + r * f32::cos(angle);
            let vy = cy + r * f32::sin(angle);
            
            let nx = (vx - x) / w - 0.5;
            let ny = (vy - y) / h - 0.5;

            let radians = gradient_angle.to_radians();
            let cos_a = radians.cos();
            let sin_a = radians.sin();

            let direction = nx * cos_a + ny * sin_a;
            let limit = 0.5 * (cos_a.abs() + sin_a.abs());

            let factor = (direction / limit * 0.5 + 0.5).clamp(0.0, 1.0);
            let v_color = Color::new(
                color_1.r + (color_2.r - color_1.r) * factor,
                color_1.g + (color_2.g - color_1.g) * factor,
                color_1.b + (color_2.b - color_1.b) * factor,
                color_1.a + (color_2.a - color_1.a) * factor,
            );

            vertices.push(Vertex {
                position: vec3(vx, vy, 0.0),
                uv: vec2((vx - x) / w, (vy - y) / h),
                color: v_color.into(),
                normal: vec4(0.0, 0.0, 1.0, 0.0)
            });
        }
    };

    add_corner_arc(x + w - r, y + r, 1.5 * PI, 2.0 * PI);
    add_corner_arc(x + w - r, y + h - r, 0.0, 0.5 * PI);
    add_corner_arc(x + r, y + h - r, 0.5 * PI, PI);
    add_corner_arc(x + r, y + r, PI, 1.5 * PI);

    let total_outer_vertices = vertices.len() - 1;
    for i in 1..total_outer_vertices {
        indices.push(0);
        indices.push(i as u16);
        indices.push((i + 1) as u16);
    }
    
    indices.push(0);
    indices.push(total_outer_vertices as u16);
    indices.push(1);

    draw_mesh(&Mesh {
        vertices,
        indices,
        texture: None,
    });
}