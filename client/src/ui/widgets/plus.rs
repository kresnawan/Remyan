use macroquad::{
    color::Color,
    math::{vec2, vec3, vec4},
    models::{Mesh, draw_mesh},
    ui::Vertex,
};

use crate::{state::State, ui::{config::{dimension::ObjectDimension, parent::ParentState, position::ObjectPosition}, traits::object::Object}};

pub struct PlusAttribute {
    pub thickness: f32,
    pub length: f32,
    pub color: Color,
}

impl PlusAttribute {
    pub fn default() -> Self {
        PlusAttribute {
            thickness: 20.0,
            length: 40.0,
            color: Color::from_rgba(255, 255, 255, 50),
        }
    }
}

pub struct Plus {
    pub position: ObjectPosition,
    pub dimension: ObjectDimension,
    pub parent: ParentState,
    pub attribute: PlusAttribute,
}

impl Plus {
    pub fn new(position: ObjectPosition, attr: PlusAttribute) -> Self {
        Plus {
            position,
            dimension: ObjectDimension::absolute(0.0, 0.0),
            parent: ParentState::new(),
            attribute: attr,
        }
    }
    pub fn default(attr: PlusAttribute) -> Plus {
        Plus {
            position: ObjectPosition::absolute(0.0, 0.0),
            dimension: ObjectDimension::absolute(0.0, 0.0),
            parent: ParentState::new(),
            attribute: attr,
        }
    }
}

impl Object for Plus {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn update(
        &mut self,
        parent_x: Option<f32>,
        parent_y: Option<f32>,
        parent_w: Option<f32>,
        parent_h: Option<f32>,
        _: &Option<State>,
    ) -> Option<State> {
        self.update_parent_state(parent_x, parent_y, parent_w, parent_h);
        self.update_dimension();
        self.update_alignment();

        return None;
    }

    fn draw(&self) {
        let mut vertices: Vec<Vertex> = Vec::new();

        let length = self.attribute.length;
        let thickness = self.attribute.thickness;
        let x = self.position.x + self.parent.x;
        let y = self.position.y + self.parent.y;

        let center_point_x = x + length + (thickness / 2.0);
        let center_point_y = y + length + (thickness / 2.0);

        let v_center = Vertex {
            position: vec3(center_point_x, center_point_y, 0.0),
            uv: vec2(0.0, 0.0),
            color: self.attribute.color.into(),
            normal: vec4(0.0, 0.0, 0.0, 0.0),
        };

        vertices.push(v_center);

        for i in vec![length, length + thickness] {
            let v = Vertex {
                position: vec3(x, i + y, 0.0),
                uv: vec2(0.0, 0.0),
                color: self.attribute.color.into(),
                normal: vec4(0.0, 0.0, 0.0, 0.0),
            };

            vertices.push(v);
        }

        for i in vec![0.0, length, length + thickness, length * 2.0 + thickness] {
            let v = Vertex {
                position: vec3(x + length, i + y, 0.0),
                uv: vec2(0.0, 0.0),
                color: self.attribute.color.into(),
                normal: vec4(0.0, 0.0, 0.0, 0.0),
            };

            vertices.push(v);
        }

        for i in vec![0.0, length, length + thickness, length * 2.0 + thickness] {
            let v = Vertex {
                position: vec3(x + length + thickness, i + y, 0.0),
                uv: vec2(0.0, 0.0),
                color: self.attribute.color.into(),
                normal: vec4(0.0, 0.0, 0.0, 0.0),
            };

            vertices.push(v);
        }

        for i in vec![length, length + thickness] {
            let v = Vertex {
                position: vec3(x + (length * 2.0) + thickness, i + y, 0.0),
                uv: vec2(0.0, 0.0),
                color: self.attribute.color.into(),
                normal: vec4(0.0, 0.0, 0.0, 0.0),
            };

            vertices.push(v);
        }

        let indices = vec![
            0, 2, 1, 0, 1, 4, 0, 2, 5, 0, 4, 3, 0, 3, 7, 0, 7, 8, 0, 8, 11, 0, 11, 12, 0, 12, 9, 0,
            9, 10, 0, 10, 6, 0, 6, 5,
        ];

        draw_mesh(&Mesh {
            vertices: vertices,
            indices: indices,
            texture: None,
        });
    }

    fn get_dimension(&self) -> ObjectDimension {
        return self.dimension.clone();
    }

    fn get_parent_state(&self) -> ParentState {
        return self.parent.clone();
    }

    fn get_position(&self) -> ObjectPosition {
        return self.position.clone();
    }

    fn set_dimension_ref(&mut self, value: ObjectDimension) {
        self.dimension = value;
    }

    fn set_parent_state_ref(&mut self, value: ParentState) {
        self.parent = value;
    }

    fn set_position_ref(&mut self, value: ObjectPosition) {
        self.position = value;
    }

    fn update_dimension(&mut self) {
        let length = self.attribute.length * 2.0 + self.attribute.thickness;

        self.set_dimension_ref(ObjectDimension {
            width: length,
            height: length,
            width_dyn: None,
            height_dyn: None,
        });
    }
}
