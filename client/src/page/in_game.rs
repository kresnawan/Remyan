use std::sync::Arc;

use macroquad::{
    camera::{Camera3D, set_camera, set_default_camera},
    color::{BLACK, DARKGRAY, GRAY},
    input::{KeyCode, is_key_released},
    math::{Vec3, vec3},
    models::draw_grid,
    rand,
    text::draw_text,
    window::clear_background,
};
use remyan_core::Deck;

use crate::{
    app::CardTextures,
    page::Page,
    state::State,
    ui::{
        three_dimensional::{card::Card, ray::get_mouse_ray},
        traits::object::Object,
    },
};

use macroquad::prelude::*;

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
    stock_pile: Vec<Card>,
    discard_pile: Vec<Card>,
    opp_1_hand: Vec<Card>,
    hand: Vec<Card>,
    hovered_card_index: Option<usize>,
    selected_card_index: Option<usize>,
    camera: Camera3D,
    is_melding: bool,
    melded_cards: Vec<Vec<Card>>,
    chosen_cards_for_meld: Vec<usize>,
}

impl InGame {
    pub fn init_stock_pile(card_textures: Arc<CardTextures>) -> Vec<Card> {
        let deck = Deck::new(true);
        let mut result: Vec<Card> = Vec::new();
        for i in deck.cards {
            let mut new_card = Card::new(
                STOCK_PILE_POS,
                STOCK_PILE_ROT,
                0.8,
                Some(&i),
                card_textures.clone(),
            );

            new_card.target_pos.y = (result.len() as f32 - 1.) * 0.002;
            result.push(new_card);
        }

        result
    }
    pub fn new(card_textures: Arc<CardTextures>) -> Self {
        let cam = Camera3D {
            position: vec3(-3.0, 4.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            target: vec3(0., 0., 0.),
            ..Default::default()
        };
        let stock_pile = InGame::init_stock_pile(card_textures.clone());
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
            stock_pile,
            selected_card_index: None,
            opp_1_hand: Vec::new(),
            hovered_card_index: None,
            camera: cam,
            is_melding: false,
            melded_cards: Vec::new(),
            chosen_cards_for_meld: Vec::new(),
        };
    }

    pub fn update_card_interaction(&mut self) {
        let ray = get_mouse_ray(&self.camera);
        let mut closest_t_index = None;

        for (i, card) in self.hand.iter().enumerate().rev() {
            if card.intersects_ray(&ray).is_some() {
                closest_t_index = Some(i);
            }
        }

        if self.hovered_card_index != closest_t_index {
            self.hovered_card_index = closest_t_index;
        }
    }

    fn meld(&mut self) {
        let mut arr: Vec<Card> = Vec::new();
        let mut previous_index = usize::MIN;

        let mut offset: usize = 0;

        for i in &self.chosen_cards_for_meld {
            let mut transformed_index = *i;

            if previous_index <= transformed_index {
                transformed_index -= offset;

                offset += 1;
            }

            let mut card = self.hand.remove(transformed_index);

            card.set_target(vec3(-1.5, 0., 0.), vec3(90., 90., 0.));

            arr.push(card);
            previous_index = *i;
        }

        self.chosen_cards_for_meld.clear();
        self.melded_cards.push(arr);
        self.rearrange_hand();
        self.rearrange_melded_cards();
        self.is_melding = false;
    }

    fn rearrange_each_melded_cards(&mut self, index: usize) {
        let spacing_x = 0.1;
        let spacing_z = 0.002;
        let count = self.melded_cards[index].len() as f32;

        let start_z = ((count - 1.0) * spacing_x) / 2.0 + (index as f32);

        let base_target_pos = vec3(-1.5, 0., start_z);

        self.melded_cards[index][0].target_pos = base_target_pos;
        self.melded_cards[index][0].target_rot = vec3(90., 90.0, 0.0);

        for i in 1..self.melded_cards[index].len() {
            let new_target_pos = Card::get_indexed_position(
                base_target_pos,
                self.melded_cards[index][0].rotation,
                i,
                spacing_x,
                spacing_z,
            );

            let card = &mut self.melded_cards[index][i];
            card.target_pos = new_target_pos;
            card.target_rot = vec3(90., 90.0, 0.0);
        }
    }

    fn rearrange_melded_cards(&mut self) {
        if self.melded_cards.is_empty() {
            return;
        }

        for i in 0..self.melded_cards.len() {
            self.rearrange_each_melded_cards(i);
        }
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

        let spacing_x = 0.3;
        let spacing_z = -0.002;
        let count = self.hand.len() as f32;

        let start_z = ((count - 1.0) * spacing_x) / 2.0;

        let base_target_pos = vec3(-1.5, 0.8 / 2., start_z);

        self.hand[0].target_pos = base_target_pos;
        self.hand[0].target_rot = vec3(self.calculate_look_at_tilt(base_target_pos) - 180., 90.0, 0.0);

        for i in 1..self.hand.len() {
            let new_target_pos = Card::get_indexed_position(
                base_target_pos,
                self.hand[0].rotation,
                i,
                spacing_x,
                spacing_z,
            );

            let x_degree = self.calculate_look_at_tilt(new_target_pos) - 180.;

            let card = &mut self.hand[i];
            card.target_pos = new_target_pos;
            card.target_rot = vec3(x_degree, 90.0, 0.0);
        }
    }

    fn rearrange_hand_side(&mut self) {
        if self.hand.is_empty() {
            return;
        }

        let spacing_x = 0.1;
        let spacing_z = 0.002;

        let base_target_pos = vec3(-1.5, 0.8 / 2., 2.);

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

    fn discard_from_hand(&mut self, index: usize) {
        if self.hand.len() - 1 >= index {
            let mut card = self.hand.remove(index);
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
        if let Some(mut new_card) = self.stock_pile.pop() {
            new_card.set_target(self.get_opp_1_hand_next_pos(), vec3(0.0, 180.0, 0.0));
            self.opp_1_hand.push(new_card);

            self.rearrange_opp_1_hand();
        }
    }

    pub fn calculate_look_at_tilt(&self, card_pos: Vec3) -> f32 {
        let delta_x = (self.cam_x - self.cam_y).abs();
        let delta_y = self.cam_y - card_pos.y;

        let tilt_angle_deg = f32::atan2(delta_x, delta_y).to_degrees();

        tilt_angle_deg
    }

    fn draw_for_hand(&mut self) {
        if let Some(mut new_card) = self.stock_pile.pop() {
            let next_pos = self.get_hand_next_pos();

            new_card.set_target(
                next_pos,
                vec3(self.calculate_look_at_tilt(next_pos), 90.0, 0.0),
            );
            self.hand.push(new_card);

            self.rearrange_hand();
        }
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

        set_camera(&self.camera);

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

        for i in &self.stock_pile {
            i.draw();
        }

        for i in &self.melded_cards {
            for card in i {
                card.draw();
            }
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

        for i in &mut self.stock_pile {
            i.update();
        }

        for i in &mut self.melded_cards {
            for card in i {
                card.update();
            }
        }

        if is_key_released(KeyCode::K) {
            self.discard_from_opp_1_hand();
        }

        if is_key_released(KeyCode::L) {
            self.draw_for_opp_1_hand();
        }

        if is_key_released(KeyCode::M) {
            self.draw_for_hand();
        }

        if is_key_released(KeyCode::O) {
            self.is_melding = !self.is_melding;
        }

        if is_key_released(KeyCode::G) {
            self.rearrange_hand_side();
        }

        if is_key_released(KeyCode::H) {
            self.rearrange_hand();
        }

        self.update_card_interaction();

        let (mx, my) = mouse_position();

        if self.is_melding {
            if is_key_released(KeyCode::P) {
                self.meld();
            }
            if is_mouse_button_released(MouseButton::Left) {
                if let Some(index) = self.hovered_card_index {
                    if self.chosen_cards_for_meld.contains(&index) {
                        let card: &mut Card = &mut self.hand[index];

                        card.target_pos.y = 0.4;
                        let mut index_to_remove: usize = 0;
                        for (i, val) in &mut self.chosen_cards_for_meld.iter().enumerate() {
                            if *val == index {
                                index_to_remove = i;
                            }
                        }
                        self.chosen_cards_for_meld.remove(index_to_remove);
                    } else {
                        let card: &mut Card = &mut self.hand[index];

                        card.target_pos.y = 0.7;
                        self.chosen_cards_for_meld.push(index);
                    }
                }
            }
        } else {
            if is_mouse_button_released(MouseButton::Left) {
                if let Some(index) = self.hovered_card_index {
                    if let Some(prev_card_index) = self.selected_card_index {
                        self.hand.swap(index, prev_card_index);
                        self.selected_card_index = None;
                        self.rearrange_hand();
                    } else {
                        self.rearrange_hand();
                        let card: &mut Card = &mut self.hand[index];

                        card.target_pos.y = 0.7;
                        self.selected_card_index = Some(index);
                    }
                } else {
                    if mx >= screen_width() / 2. - 100.
                        && mx <= screen_width() / 2. + 100.0
                        && my >= screen_height() / 2. - 200.
                        && my <= screen_height() / 2. + 50.
                    {
                        if let Some(index) = self.selected_card_index {
                            self.discard_from_hand(index);
                            self.selected_card_index = None;
                        }
                    } else {
                        self.rearrange_hand();
                        self.selected_card_index = None;
                    }
                }
            }
        }

        return None;
    }
}
