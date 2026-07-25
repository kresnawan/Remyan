use macroquad::prelude::*;

pub struct Card {
    pub position: Vec3,
    pub rotation: Vec3,
    pub dimension: Vec2,
    pub target_pos: Vec3,
    pub target_rot: Vec3,
    pub target_dim: Vec2,
    pub is_animating: bool,
    z_offset: f32,
    front_local_vertices: [(Vec3, Vec2); 4],
    back_local_vertices: [(Vec3, Vec2); 4],
    front_texture: Texture2D,
    back_texture: Texture2D,
}

impl Card {
    pub fn draw_debug(&self) {
        draw_text(
            &format!(
                "Current card pos X: {}, Y: {}, Z: {}",
                self.position.x, self.position.y, self.position.z
            ),
            10.0,
            100.0,
            30.0,
            BLACK,
        );
        draw_text(
            &format!(
                "Current card rot X: {}, Y: {}, Z: {}",
                self.rotation.x, self.rotation.y, self.rotation.z
            ),
            10.0,
            120.0,
            30.0,
            BLACK,
        );
    }

    pub fn set_position_ref(&mut self, pos: Vec3) {
        self.position = pos;
    }

    pub fn set_target(&mut self, pos: Vec3, rot: Vec3) {
        self.target_pos = pos;
        self.target_rot = rot;

        self.is_animating = true;
    }

    pub fn set_target_with_dim(&mut self, pos: Vec3, rot: Vec3, height: f32) {
        self.target_pos = pos;
        self.target_rot = rot;
        self.target_dim.x = 2.5 / 3.5 * (height / 2.);
        self.target_dim.y = height / 2.;

        self.is_animating = true;
    }

    pub fn new(position: Vec3, rotation: Vec3, height: f32, card_back_texture: &Texture2D) -> Self {
        let img_f = Image::gen_image_color(200, 300, BLUE);

        let front_texture = Texture2D::from_image(&img_f);
        let back_texture = card_back_texture;

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
            target_pos: position,
            target_rot: rotation,
            front_local_vertices,
            back_local_vertices,
            front_texture: front_texture,
            back_texture: back_texture.clone(),
            dimension: vec2(w, h),
            target_dim: vec2(w, h),
            z_offset: z_offset,
            is_animating: false,
        }
    }

    pub fn update(&mut self) {
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

        let dt = get_frame_time();
        let speed = 10.0;
        self.position = self.position.lerp(self.target_pos, speed * dt);
        self.rotation = self.rotation.lerp(self.target_rot, speed * dt);
        self.dimension = self.dimension.lerp(self.target_dim, speed * dt);

        if self.position.distance(self.target_pos) < 0.0001 {
            self.position = self.target_pos;
            self.rotation = self.target_rot;

            self.is_animating = false;
        }
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
        index: usize,
        spacing_x: f32,
        spacing_z: f32,
    ) -> Vec3 {
        let rotation = Mat4::from_rotation_y(rotation_angles.y.to_radians())
            * Mat4::from_rotation_x(rotation_angles.x.to_radians())
            * Mat4::from_rotation_z(rotation_angles.z.to_radians());

        let local_offset = vec3(index as f32 * spacing_x, 0.0, index as f32 * spacing_z);

        let rotated_offset = rotation.transform_vector3(local_offset);
        base_position + rotated_offset
    }
}
