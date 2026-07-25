use macroquad::{
    camera::{Camera3D, set_camera, set_default_camera},
    color::{BLACK, DARKGRAY, GRAY},
    input::{KeyCode, is_key_released},
    math::{Vec3, vec3},
    models::draw_grid,
    rand,
    text::draw_text,
    texture::Texture2D,
    window::clear_background,
};

use crate::{
    page::Page,
    state::State,
    ui::{three_dimensional::card::Card, traits::object::Object},
};

const STOCK_PILE_POS: Vec3 = vec3(2.0, 0., -3.);
const STOCK_PILE_ROT: Vec3 = vec3(90.0, 0.0, 0.0);

pub struct InGame {
    objects: Vec<Box<dyn Object>>,
    cam_x: f32,
    cam_y: f32,
    cam_z: f32,
    cam_up_x: f32,
    cam_up_y: f32,
    cam_up_z: f32,
    cam_target_x: f32,
    cam_target_y: f32,
    cam_target_z: f32,
    is_started: bool,
    stock_pile_card: Card,
    discard_pile: Vec<Card>,
    opp_1_hand: Vec<Card>,
    hand: Vec<Card>,
    card_back_texture: Texture2D,
}

impl InGame {
    pub fn new(card_back_texture: &Texture2D) -> Self {
        return Self {
            discard_pile: Vec::new(),
            objects: Vec::new(),
            cam_x: -3.0,
            // cam_y: 0.,
            hand: Vec::new(),
            cam_y: 4.,
            cam_z: 0.0,
            cam_up_x: 0.0,
            cam_up_y: 1.0,
            cam_up_z: 0.0,
            cam_target_x: 0.0,
            cam_target_y: 0.0,
            cam_target_z: 0.0,
            is_started: false,
            card_back_texture: card_back_texture.clone(),
            stock_pile_card: Card::new(STOCK_PILE_POS, STOCK_PILE_ROT, 0.8, card_back_texture),
            opp_1_hand: Vec::new(),
        };
    }

    fn rearrange_opp_1_hand(&mut self) {
        if self.opp_1_hand.is_empty() {
            return;
        }

        let spacing_x = 0.16;
        let spacing_z = 0.002;
        let count = self.opp_1_hand.len() as f32;

        let start_x = ((count - 1.0) * -spacing_x) / 2.0;

        let base_target_pos = vec3(start_x, 0.8 / 2., -2.5);

        self.opp_1_hand[0].target_pos = base_target_pos;
        self.opp_1_hand[0].target_rot = vec3(0.0, 180.0, 0.0);

        for i in 1..self.opp_1_hand.len() {
            let new_target_pos = Card::get_indexed_position(
                base_target_pos,
                self.opp_1_hand[0].rotation,
                i,
                -spacing_x,
                spacing_z,
            );

            let card = &mut self.opp_1_hand[i];
            card.target_pos = new_target_pos;
            card.target_rot = vec3(0.0, 180.0, 0.0);
        }
    }

    fn rearrange_hand(&mut self) {
        if self.hand.is_empty() {
            return;
        }

        let spacing_x = 0.2;
        let spacing_z = 0.002;
        let count = self.hand.len() as f32;

        let start_z = ((count - 1.0) * spacing_x) / 2.0;

        let base_target_pos = vec3(-1.5, 0.8 / 2., start_z);

        self.hand[0].target_pos = base_target_pos;
        self.hand[0].target_rot = vec3(self.calculate_look_at_tilt(base_target_pos), 90.0, 0.0);

        for i in 1..self.hand.len() {
            let new_target_pos = Card::get_indexed_position(
                base_target_pos,
                self.hand[0].rotation,
                i,
                spacing_x,
                spacing_z,
            );

            let x_degree = self.calculate_look_at_tilt(new_target_pos);

            let card = &mut self.hand[i];
            card.target_pos = new_target_pos;
            card.target_rot = vec3(x_degree, 90.0, 0.0);
        }
    }

    fn discard_from_opp_1_hand(&mut self) {
        if let Some(mut card) = self.opp_1_hand.pop() {
            let rand_rot_y: f32 = rand::gen_range(0.0, 360.0);
            let rand_x: f32 = rand::gen_range(-0.125, 0.125);
            let rand_z: f32 = rand::gen_range(-0.125, 0.125);

            card.set_target_with_dim(
                vec3(rand_x, self.discard_pile.len() as f32 * 0.001, rand_z),
                vec3(-90.0, rand_rot_y, 0.0),
                1.2,
            );
            self.discard_pile.push(card);
            self.rearrange_opp_1_hand();
        }
    }

    fn discard_from_hand(&mut self) {
        if let Some(mut card) = self.hand.pop() {
            let rand_rot_y: f32 = rand::gen_range(0.0, 360.0);
            let rand_x: f32 = rand::gen_range(-0.125, 0.125);
            let rand_z: f32 = rand::gen_range(-0.125, 0.125);

            card.set_target_with_dim(
                vec3(rand_x, self.discard_pile.len() as f32 * 0.001, rand_z),
                vec3(-90.0, rand_rot_y, 0.0),
                1.2,
            );
            self.discard_pile.push(card);
            self.rearrange_hand();
        }
    }

    fn draw_for_opp_1_hand(&mut self) {
        let mut new_card = Card::new(STOCK_PILE_POS, STOCK_PILE_ROT, 0.8, &self.card_back_texture);

        new_card.set_target(self.get_opp_1_hand_next_pos(), vec3(0.0, 180.0, 0.0));
        self.opp_1_hand.push(new_card);

        self.rearrange_opp_1_hand();
    }

    pub fn calculate_look_at_tilt(&self, card_pos: Vec3) -> f32 {
        let delta_x = (self.cam_x - self.cam_y).abs();
        let delta_y = self.cam_y - card_pos.y;

        let tilt_angle_deg = f32::atan2(delta_x, delta_y).to_degrees();

        tilt_angle_deg
    }

    fn draw_for_hand(&mut self) {
        let mut new_card = Card::new(STOCK_PILE_POS, STOCK_PILE_ROT, 0.8, &self.card_back_texture);
        let next_pos = self.get_hand_next_pos();

        new_card.set_target(
            next_pos,
            vec3(self.calculate_look_at_tilt(next_pos), 90.0, 0.0),
        );
        self.hand.push(new_card);

        self.rearrange_hand();
    }

    fn get_opp_1_hand_next_pos(&self) -> Vec3 {
        if self.opp_1_hand.is_empty() {
            return vec3(0.0, 0.8 / 2., -2.5);
        }
        let pos = &self.opp_1_hand[self.opp_1_hand.len() - 1].target_pos;

        let mut cloned = pos.clone();
        cloned.x += 0.016;

        cloned
    }

    fn get_hand_next_pos(&self) -> Vec3 {
        if self.hand.is_empty() {
            return vec3(-1.5, 2., 0.0);
        }
        let pos = &self.hand[self.hand.len() - 1].target_pos;

        let mut cloned = pos.clone();
        cloned.z += 0.016;

        cloned
    }

    fn on_start(&mut self) {
        self.rearrange_opp_1_hand();
    }
}

impl Page for InGame {
    fn draw(&self) {
        clear_background(DARKGRAY);
        for i in &self.objects {
            i.draw();
        }

        set_camera(&Camera3D {
            position: vec3(self.cam_x, self.cam_y, self.cam_z),
            up: vec3(self.cam_up_x, self.cam_up_y, self.cam_up_z),
            target: vec3(self.cam_target_x, self.cam_target_y, self.cam_target_z),
            ..Default::default()
        });

        self.stock_pile_card.draw();
        draw_grid(20, 1., BLACK, GRAY);

        for i in &self.opp_1_hand {
            i.draw();
        }

        for i in &self.hand {
            i.draw();
        }

        for i in &self.discard_pile {
            i.draw();
        }

        set_default_camera();
        draw_text("WELCOME TO 3D WORLD", 10.0, 20.0, 30.0, BLACK);
        draw_text(
            &format!(
                "Cam X: {}, Y: {}, Z: {}",
                self.cam_x, self.cam_y, self.cam_z
            ),
            10.0,
            40.0,
            30.0,
            BLACK,
        );
        draw_text(
            &format!(
                "Cam Up X: {}, Y: {}, Z: {}",
                self.cam_up_x, self.cam_up_y, self.cam_up_z
            ),
            10.0,
            60.0,
            30.0,
            BLACK,
        );
        draw_text(
            &format!(
                "Cam Target X: {}, Y: {}, Z: {}",
                self.cam_target_x, self.cam_target_y, self.cam_target_z
            ),
            10.0,
            80.0,
            30.0,
            BLACK,
        );
    }

    fn update(&mut self, state: &Option<State>) -> Option<State> {
        if !self.is_started {
            self.on_start();

            println!("Hello");

            self.is_started = true;
        }

        for i in &mut self.objects {
            if let Some(state) = i.update(None, None, None, None, state) {
                return Some(state);
            }
        }

        for i in &mut self.opp_1_hand {
            i.update();
        }

        for i in &mut self.hand {
            i.update();
        }

        for i in &mut self.discard_pile {
            i.update();
        }

        if is_key_released(KeyCode::K) {
            self.discard_from_opp_1_hand();
        }

        if is_key_released(KeyCode::L) {
            self.draw_for_opp_1_hand();
        }

        if is_key_released(KeyCode::N) {
            self.discard_from_hand();
        }

        if is_key_released(KeyCode::M) {
            self.draw_for_hand();
        }

        self.stock_pile_card.update();

        return None;
    }
}
