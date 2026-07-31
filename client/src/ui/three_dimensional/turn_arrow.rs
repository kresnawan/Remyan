use crate::{app::CardTextures, ui::config::position};
use macroquad::prelude::*;
use remyan_core::Card;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct TurnArrow {
    pub position: Vec3,
    pub rotation: Vec3,
    pub dimension: Vec2,

    pub target_pos: Vec3,
    pub target_rot: Vec3,
    pub target_dim: Vec2,

    pub start_pos: Vec3,
    pub start_rot: Vec3,
    pub start_dim: Vec2,
    pub animation_time: f32,
    pub animation_duration: f32,

    pub is_animating: bool,
    front_local_vertices: [(Vec3, Vec2); 4],
    pub front_texture: Texture2D,
}

fn ease_out_cubic(t: f32) -> f32 {
    let p = 1.0 - t;
    1.0 - p * p * p
}

impl TurnArrow {
    pub fn set_target(&mut self, rot: Vec3) {
        self.start_rot = self.rotation;

        self.target_rot = rot;

        self.animation_time = 0.;
        self.is_animating = true;
    }

    pub fn new(textures: Arc<CardTextures>) -> Self {
        let front_texture = textures.arrow_texture.clone();

        front_texture.set_filter(FilterMode::Linear);

        let position = vec3(0., -0.01, 0.);
        let rotation = vec3(90., 360., 0.);

        let h: f32 = 2.3;
        let w = h;

        let z_offset = 0.;

        let front_local_vertices = [
            (vec3(-w, -h, z_offset), vec2(0.0, 1.0)),
            (vec3(w, -h, z_offset), vec2(1.0, 1.0)),
            (vec3(w, h, z_offset), vec2(1.0, 0.0)),
            (vec3(-w, h, z_offset), vec2(0.0, 0.0)),
        ];

        Self {
            position,
            rotation,
            dimension: vec2(w, h),

            target_pos: position,
            target_rot: rotation,
            target_dim: vec2(w, h),

            start_pos: position,
            start_rot: rotation,
            start_dim: vec2(w, h),

            animation_time: 0.,
            animation_duration: 0.5,

            front_local_vertices,
            front_texture: front_texture,

            is_animating: true,
        }
    }

    pub fn update(&mut self) {
        if self.is_animating {
            let dt = get_frame_time();
            self.animation_time += dt;

            let progress = (self.animation_time / self.animation_duration).clamp(0., 1.);
            let eased_t = ease_out_cubic(progress);

            self.rotation = self.start_rot.lerp(self.target_rot, eased_t);

            if self.rotation.distance(self.target_rot) < 0.0001 {
                self.rotation = self.target_rot;

                self.is_animating = false;
            }
        }

        self.front_local_vertices = [
            (
                vec3(-self.dimension.x, -self.dimension.y, 0.),
                vec2(0.0, 1.0),
            ),
            (
                vec3(self.dimension.x, -self.dimension.y, 0.),
                vec2(1.0, 1.0),
            ),
            (vec3(self.dimension.x, self.dimension.y, 0.), vec2(1.0, 0.0)),
            (
                vec3(-self.dimension.x, self.dimension.y, 0.),
                vec2(0.0, 0.0),
            ),
        ];
    }

    pub fn draw(&self) {
        let rotation = Mat4::from_rotation_y(self.rotation.y.to_radians())
            * Mat4::from_rotation_x(self.rotation.x.to_radians())
            * Mat4::from_rotation_z(self.rotation.z.to_radians());

        let build_mesh = |vertices: &[(Vec3, Vec2)], texture: Texture2D| -> Mesh {
            let transformed_vertices = vertices
                .iter()
                .map(|(pos, uv)| Vertex {
                    position: rotation.transform_point3(*pos) + self.position,
                    uv: *uv,
                    color: Color::from_rgba(255, 255, 255, (0.50 * 255 as f32) as u8).into(),
                    normal: vec4(0.0, 0.0, 0.0, 0.0),
                })
                .collect();

            Mesh {
                vertices: transformed_vertices,
                indices: vec![0, 1, 2, 0, 2, 3],
                texture: Some(texture),
            }
        };

        let front_mesh = build_mesh(&self.front_local_vertices, self.front_texture.clone());

        draw_mesh(&front_mesh);
    }
}
