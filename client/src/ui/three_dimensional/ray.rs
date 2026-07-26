use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

pub fn get_mouse_ray(camera: &Camera3D) -> Ray {
    let (mx, my) = mouse_position();
    let sw = screen_width();
    let sh = screen_height();

    let x_ndc = (2.0 * mx) / sw - 1.0;
    let y_ndc = 1.0 - (2.0 * my) / sh;

    let forward = (camera.target - camera.position).normalize();
    let right = forward.cross(camera.up).normalize();
    let up = right.cross(forward).normalize();

    let aspect = sw / sh;
    let fovy_rad = 45.0f32.to_radians();
    let tan_fovy = (fovy_rad / 2.0).tan();

    let ray_dir = (forward + right * (x_ndc * aspect * tan_fovy) + up * (y_ndc * tan_fovy)).normalize();

    Ray {
        origin: camera.position,
        dir: ray_dir,
    }
}