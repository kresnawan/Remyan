use crate::{app::CardTextures, ui::three_dimensional::ray::Ray};
use macroquad::prelude::*;
use remyan_core::Card;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct CardElement {
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
    z_offset: f32,
    front_local_vertices: [(Vec3, Vec2); 4],
    back_local_vertices: [(Vec3, Vec2); 4],
    pub front_texture: Texture2D,
    pub back_texture: Texture2D,
    card_textures: Arc<CardTextures>,
    pub card: Option<Card>
}

fn ease_out_cubic(t: f32) -> f32 {
    let p = 1.0 - t;
    1.0 - p * p * p
}

fn ease_in_cubic(t: f32) -> f32 {
    t * t
}

impl CardElement {
    pub fn set_position_ref(&mut self, pos: Vec3) {
        self.position = pos;
    }

    pub fn set_card(&mut self, card: Option<&Card>) {
        if let Some(card) = card {
            self.card = Some(card.clone());
            self.front_texture = self.card_textures.get(card);
        } else {
            self.card = None;
            self.front_texture = self.card_textures.get_empty_texture();
        }
    }

    pub fn set_target(&mut self, pos: Vec3, rot: Vec3) {
        self.start_pos = self.position;
        self.start_rot = self.rotation;
        self.start_dim = self.dimension;

        self.target_pos = pos;
        self.target_rot = rot;

        self.animation_time = 0.;
        self.is_animating = true;
    }

    pub fn set_target_with_dim(&mut self, pos: Vec3, rot: Vec3, height: f32) {
        self.start_pos = self.position;
        self.start_rot = self.rotation;
        self.start_dim = self.dimension;

        self.target_pos = pos;
        self.target_rot = rot;
        self.target_dim.x = 2.5 / 3.5 * (height / 2.);
        self.target_dim.y = height / 2.;

        self.animation_time = 0.;
        self.is_animating = true;
    }

    pub fn new(
        position: Vec3,
        rotation: Vec3,
        height: f32,
        card: Option<&remyan_core::Card>,
        card_textures: Arc<CardTextures>,
    ) -> Self {
        let front_texture: Texture2D;
        let cloned_card: Option<Card>;
        if let Some(card) = card {
            front_texture = card_textures.get(card);
            cloned_card = Some(card.clone())
        } else {
            front_texture = card_textures.get_empty_texture();
            cloned_card = None;
        }
        let back_texture = card_textures.get_back_texture();

        front_texture.set_filter(FilterMode::Linear);
        back_texture.set_filter(FilterMode::Linear);

        let h: f32 = height / 2.;
        let w = (2.5 / 3.5) * h;

        let z_offset = 0.0005;

        let front_local_vertices = [
            (vec3(-w, -h, z_offset), vec2(0.0, 1.0)),
            (vec3(w, -h, z_offset), vec2(1.0, 1.0)),
            (vec3(w, h, z_offset), vec2(1.0, 0.0)),
            (vec3(-w, h, z_offset), vec2(0.0, 0.0)),
        ];

        let back_local_vertices = [
            (vec3(w, -h, -z_offset), vec2(0.0, 1.0)),
            (vec3(-w, -h, -z_offset), vec2(1.0, 1.0)),
            (vec3(-w, h, -z_offset), vec2(1.0, 0.0)),
            (vec3(w, h, -z_offset), vec2(0.0, 0.0)),
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
            back_local_vertices,
            front_texture: front_texture,
            back_texture: back_texture.clone(),

            z_offset: z_offset,
            is_animating: true,
            card_textures,
            card: cloned_card,
        }
    }

    pub fn update(&mut self) {
        if self.is_animating {
            let dt = get_frame_time();
            self.animation_time += dt;

            let progress = (self.animation_time / self.animation_duration).clamp(0., 1.);
            let eased_t = ease_out_cubic(progress);

            self.position = self.start_pos.lerp(self.target_pos, eased_t);
            self.rotation = self.start_rot.lerp(self.target_rot, eased_t);
            self.dimension = self.start_dim.lerp(self.target_dim, eased_t);

            if self.position.distance(self.target_pos) < 0.0001 {
                self.position = self.target_pos;
                self.rotation = self.target_rot;
                self.dimension = self.target_dim;

                self.is_animating = false;
            }
        }

        self.front_local_vertices = [
            (
                vec3(-self.dimension.x, -self.dimension.y, self.z_offset),
                vec2(0.0, 1.0),
            ),
            (
                vec3(self.dimension.x, -self.dimension.y, self.z_offset),
                vec2(1.0, 1.0),
            ),
            (
                vec3(self.dimension.x, self.dimension.y, self.z_offset),
                vec2(1.0, 0.0),
            ),
            (
                vec3(-self.dimension.x, self.dimension.y, self.z_offset),
                vec2(0.0, 0.0),
            ),
        ];

        self.back_local_vertices = [
            (
                vec3(self.dimension.x, -self.dimension.y, -self.z_offset),
                vec2(0.0, 1.0),
            ),
            (
                vec3(-self.dimension.x, -self.dimension.y, -self.z_offset),
                vec2(1.0, 1.0),
            ),
            (
                vec3(-self.dimension.x, self.dimension.y, -self.z_offset),
                vec2(1.0, 0.0),
            ),
            (
                vec3(self.dimension.x, self.dimension.y, -self.z_offset),
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
                    color: WHITE.into(),
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
        let back_mesh = build_mesh(&self.back_local_vertices, self.back_texture.clone());

        draw_mesh(&front_mesh);
        draw_mesh(&back_mesh);
    }

    pub fn get_indexed_position(
        base_position: Vec3,
        rotation_angles: Vec3,
        index: f32,
        spacing_x: f32,
        spacing_z: f32,
    ) -> Vec3 {
        let rotation = Mat4::from_rotation_y(rotation_angles.y.to_radians())
            * Mat4::from_rotation_x(rotation_angles.x.to_radians())
            * Mat4::from_rotation_z(rotation_angles.z.to_radians());

        let local_offset = vec3(index as f32 * spacing_x, 0.0, index * spacing_z);

        let rotated_offset = rotation.transform_vector3(local_offset);
        base_position + rotated_offset
    }

    pub fn intersects_ray(&self, ray: &Ray) -> Option<Vec3> {
        let rotation = Mat4::from_rotation_y(self.rotation.y.to_radians())
            * Mat4::from_rotation_x(self.rotation.x.to_radians())
            * Mat4::from_rotation_z(self.rotation.z.to_radians());

        let normal = rotation.transform_vector3(vec3(0.0, 0.0, 1.0)).normalize();
        let denominator = ray.dir.dot(normal);

        if denominator.abs() < 0.0001 {
            return None;
        }

        let t = (self.position - ray.origin).dot(normal) / denominator;
        if t < 0.0 {
            return None;
        }

        let hit_point = ray.origin + ray.dir * t;

        let local_hit = rotation
            .inverse()
            .transform_vector3(hit_point - self.position);

        let half_w = self.dimension.x;
        let half_h = self.dimension.y;

        if local_hit.x.abs() <= half_w && local_hit.y.abs() <= half_h {
            Some(hit_point)
        } else {
            None
        }
    }
}
