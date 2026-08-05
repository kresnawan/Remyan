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
use remyan_core::{
    Card, NumberOfJokers, protocol::{
        DrawSource,
        command::{GameCommand, TurnCommand},
        event::{GameEvent, TurnEvent},
    },
};

use crate::{
    app::CardTextures,
    page::Page,
    state::State,
    ui::{
        config::parent::ParentState,
        three_dimensional::{card::CardElement, ray::get_mouse_ray, turn_arrow::TurnArrow},
        traits::object::Object,
    },
};

use macroquad::prelude::*;

const STOCK_PILE_POS: Vec3 = vec3(2.0, 0., -3.);
const STOCK_PILE_ROT: Vec3 = vec3(90.0, 0.0, 0.0);

enum PlayerPlacement {
    Left,
    Front,
    Right,
    Me,
}

pub struct InGame {
    objects: Vec<Box<dyn Object>>,
    self_id: u32,
    is_sharing_card: Option<PlayerPlacement>,
    shared_cards: u8,
    player_turns: Vec<u32>,
    self_index: usize,
    stock_number: usize,
    each_hand: usize,
    stock_pile: Vec<CardElement>,
    discard_pile: Vec<CardElement>,
    opp_1_hand: Vec<CardElement>,
    opp_2_hand: Vec<CardElement>,
    opp_3_hand: Vec<CardElement>,
    hand: Vec<CardElement>,
    hovered_card_index: Option<usize>,
    selected_card_index: Option<usize>,
    camera: Camera3D,
    is_hover_stock_pile: bool,
    is_melding: bool,
    melded_cards: Vec<CardElement>,
    opp_1_melded_cards: Vec<CardElement>,
    opp_2_melded_cards: Vec<CardElement>,
    opp_3_melded_cards: Vec<CardElement>,
    selected_cards_for_meld: Vec<usize>,
    self_cards: Vec<Card>,
    current_index_share: usize,
    elapsed_time: f32,
    target_time: f32,
    turn_arrow: TurnArrow,
}

impl InGame {
    pub fn new(card_textures: Arc<CardTextures>, self_id: u32, joker: &Option<NumberOfJokers>) -> Self {
        let cam = Camera3D {
            position: vec3(-3.5, 4.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            target: vec3(0., 0., 0.),
            ..Default::default()
        };

        let stock_pile = InGame::init_stock_pile(card_textures.clone(), joker.clone());
        return Self {
            discard_pile: Vec::new(),
            player_turns: Vec::new(),
            self_id,
            self_index: 0,
            objects: Vec::new(),
            hand: Vec::new(),
            is_sharing_card: None,
            shared_cards: 0,
            stock_number: 0,
            each_hand: 0,
            stock_pile,
            is_hover_stock_pile: false,
            selected_card_index: None,
            opp_1_hand: Vec::new(),
            opp_2_hand: Vec::new(),
            opp_3_hand: Vec::new(),
            self_cards: Vec::new(),
            hovered_card_index: None,
            camera: cam,
            current_index_share: 0,
            elapsed_time: 0.,
            is_melding: false,
            melded_cards: Vec::new(),
            opp_1_melded_cards: Vec::new(),
            opp_2_melded_cards: Vec::new(),
            opp_3_melded_cards: Vec::new(),
            selected_cards_for_meld: Vec::new(),
            turn_arrow: TurnArrow::new(card_textures),
            target_time: 0.0,
        };
    }

    fn init_stock_pile(card_textures: Arc<CardTextures>, joker: Option<NumberOfJokers>) -> Vec<CardElement> {
        let mut result: Vec<CardElement> = Vec::new();
        let base_number_of_card = 10;
        // if let Some(num) = joker {
        //     base_number_of_card += num.as_number()
        // }

        for _ in 0..base_number_of_card {
            let mut new_card = CardElement::new(
                STOCK_PILE_POS,
                STOCK_PILE_ROT,
                0.8,
                None,
                card_textures.clone(),
            );

            new_card.set_target(
                vec3(
                    new_card.position.x,
                    (result.len() as f32 - 1.) * 0.002,
                    new_card.position.z,
                ),
                new_card.rotation,
            );
            result.push(new_card);
        }

        result
    }

    fn get_placement(&self, player_id: u32) -> Option<PlayerPlacement> {
        let n = self.player_turns.len();

        if n == 4 {
            if self.player_turns[(self.self_index + 1) % n] == player_id {
                Some(PlayerPlacement::Right)
            } else if self.player_turns[(self.self_index + 2) % n] == player_id {
                Some(PlayerPlacement::Front)
            } else if self.player_turns[(self.self_index + 3) % n] == player_id {
                Some(PlayerPlacement::Left)
            } else if player_id == self.self_id {
                Some(PlayerPlacement::Me)
            } else {
                None
            }
        } else if n == 3 {
            if self.self_index == 1 {
                if self.player_turns[0] == player_id {
                    Some(PlayerPlacement::Left)
                } else if self.player_turns[2] == player_id {
                    Some(PlayerPlacement::Right)
                } else {
                    Some(PlayerPlacement::Me)
                }
            } else if self.self_index == 0 {
                if self.player_turns[1] == player_id {
                    Some(PlayerPlacement::Right)
                } else if self.player_turns[2] == player_id {
                    Some(PlayerPlacement::Front)
                } else {
                    Some(PlayerPlacement::Me)
                }
            } else if self.self_index == 2 {
                if self.player_turns[1] == player_id {
                    Some(PlayerPlacement::Left)
                } else if self.player_turns[0] == player_id {
                    Some(PlayerPlacement::Front)
                } else {
                    Some(PlayerPlacement::Me)
                }
            } else {
                None
            }
        } else {
            return None;
        }
    }

    pub fn update_stock_pile_hover(&mut self) {
        let ray = get_mouse_ray(&self.camera);
        let mut is_hovering = false;

        let mut arr = self.stock_pile.iter().rev().peekable();
        let top_card = arr.peek();

        if let Some(card) = top_card {
            let a = card.intersects_ray(&ray);
            if let Some(_) = a {
                is_hovering = true;
            }
        }

        self.is_hover_stock_pile = is_hovering;
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
        self.selected_cards_for_meld
            .sort_unstable_by(|a, b| b.cmp(a));

        for i in &self.selected_cards_for_meld {
            let card = self.hand.remove(*i);

            // card.set_target(vec3(-1.5, 0., 0.), vec3(90., 90., 0.));

            self.melded_cards.push(card);
        }

        self.selected_cards_for_meld.clear();

        self.rearrange_hand();
        self.rearrange_melded_cards();
        self.is_melding = false;
    }

    fn meld_opp_1_cards(&mut self, cards: Vec<Card>) {
        for card in cards {
            if let Some(mut card_element) = self.opp_1_hand.pop() {
                card_element.set_card(Some(&card));
                self.opp_1_melded_cards.push(card_element);
            }
        }

        self.rearrange_opp_1_hand();
        self.rearrange_opp_1_melded_cards();
    }

    fn meld_opp_2_cards(&mut self, cards: Vec<Card>) {
        for card in cards {
            if let Some(mut card_element) = self.opp_2_hand.pop() {
                card_element.set_card(Some(&card));
                self.opp_2_melded_cards.push(card_element);
            }
        }

        self.rearrange_opp_2_hand();
        self.rearrange_opp_2_melded_cards();
    }

    fn meld_opp_3_cards(&mut self, cards: Vec<Card>) {
        for card in cards {
            if let Some(mut card_element) = self.opp_3_hand.pop() {
                card_element.set_card(Some(&card));
                self.opp_3_melded_cards.push(card_element);
            }
        }

        self.rearrange_opp_3_hand();
        self.rearrange_opp_3_melded_cards();
    }

    fn rearrange_melded_cards(&mut self) {
        if self.melded_cards.is_empty() {
            return;
        }

        let spacing_x = -0.1;
        let spacing_z = 0.002;
        let count = self.melded_cards.len() as f32;

        let start_x = ((count - 1.0) * spacing_x) / 2.0;
        let base_target_pos = vec3(-1.5, 0., start_x);

        self.melded_cards[0].set_target(base_target_pos, vec3(270., 90., 0.0));

        for i in 1..self.melded_cards.len() {
            let new_target_pos = CardElement::get_indexed_position(
                base_target_pos,
                self.melded_cards[0].rotation,
                i as f32,
                spacing_x,
                spacing_z,
            );

            let card = &mut self.melded_cards[i];
            card.set_target(new_target_pos, vec3(270., 90., 0.0));
        }
    }


    fn drop_opp_1_hand(&mut self, mut cards: Vec<Card>) {
        if self.opp_1_hand.is_empty() {
            return;
        }

        let spacing_x = 0.1;
        let spacing_z = 0.002;
        let count = self.opp_1_hand.len() as f32;

        let start_x = ((count - 1.0) * spacing_x) / 2.0;
        let base_target_pos = vec3(start_x, 0., -2.5);
        let target_rot = vec3(-90., 0., 180.0);

        self.opp_1_hand[0].set_card(Some(&cards.pop().unwrap()));
        self.opp_1_hand[0].set_target(base_target_pos, target_rot);

        for i in 1..self.opp_1_hand.len() {
            let new_target_pos = CardElement::get_indexed_position(
                base_target_pos,
                target_rot,
                i as f32,
                spacing_x,
                spacing_z,
            );

            let card = &mut self.opp_1_hand[i];
            card.set_card(Some(&cards.pop().unwrap()));
            card.set_target(new_target_pos, target_rot);
        }
    }

    fn drop_opp_2_hand(&mut self, mut cards: Vec<Card>) {
        if self.opp_2_hand.is_empty() {
            return;
        }

        let spacing_x = 0.1;
        let spacing_z = 0.002;
        let count = self.opp_2_hand.len() as f32;

        let start_x = ((count - 1.0) * -spacing_x) / 2.0;
        let base_target_pos = vec3(start_x, 0., 2.5);
        let target_rot = vec3(-90., 0., 0.0);

        self.opp_2_hand[0].set_card(Some(&cards.pop().unwrap()));
        self.opp_2_hand[0].set_target(base_target_pos, target_rot);

        for i in 1..self.opp_2_hand.len() {
            let new_target_pos = CardElement::get_indexed_position(
                base_target_pos,
                target_rot,
                i as f32,
                spacing_x,
                spacing_z,
            );

            let card = &mut self.opp_2_hand[i];
            card.set_card(Some(&cards.pop().unwrap()));
            card.set_target(new_target_pos, target_rot);
        }
    }

    fn drop_opp_3_hand(&mut self, mut cards: Vec<Card>) {
        if self.opp_3_hand.is_empty() {
            return;
        }

        let spacing_x = 0.1;
        let spacing_z = 0.002;
        let count = self.opp_3_hand.len() as f32;

        let start_x = ((count - 1.0) * spacing_x) / 2.0;
        let base_target_pos = vec3(2.5, 0., start_x);
        let target_rot = vec3(-90., 90., 0.0);

        self.opp_3_hand[0].set_card(Some(&cards.pop().unwrap()));
        self.opp_3_hand[0].set_target(base_target_pos, target_rot);

        for i in 1..self.opp_3_hand.len() {
            let new_target_pos = CardElement::get_indexed_position(
                base_target_pos,
                target_rot,
                i as f32,
                spacing_x,
                spacing_z,
            );

            let card = &mut self.opp_3_hand[i];
            card.set_card(Some(&cards.pop().unwrap()));
            card.set_target(new_target_pos, target_rot);
        }
    }

    fn rearrange_opp_1_melded_cards(&mut self) {
        if self.opp_1_melded_cards.is_empty() {
            return;
        }

        let spacing_x = 0.1;
        let spacing_z = 0.002;
        let count = self.opp_1_melded_cards.len() as f32;

        let start_x = ((count - 1.0) * spacing_x) / 2.0;
        let base_target_pos = vec3(start_x, 0., -1.5);
        let target_rot = vec3(-90., 0., 180.0);

        self.opp_1_melded_cards[0].set_target(base_target_pos, target_rot);

        for i in 1..self.opp_1_melded_cards.len() {
            let new_target_pos = CardElement::get_indexed_position(
                base_target_pos,
                target_rot,
                i as f32,
                spacing_x,
                spacing_z,
            );

            let card = &mut self.opp_1_melded_cards[i];
            card.set_target(new_target_pos, target_rot);
        }
    }

    fn rearrange_opp_2_melded_cards(&mut self) {
        if self.opp_2_melded_cards.is_empty() {
            return;
        }

        let spacing_x = 0.1;
        let spacing_z = 0.002;
        let count = self.opp_2_melded_cards.len() as f32;

        let start_x = ((count - 1.0) * -spacing_x) / 2.0;
        let base_target_pos = vec3(start_x, 0., 1.5);
        let target_rot = vec3(-90., 0., 0.0);

        self.opp_2_melded_cards[0].set_target(base_target_pos, target_rot);

        for i in 1..self.opp_2_melded_cards.len() {
            let new_target_pos = CardElement::get_indexed_position(
                base_target_pos,
                target_rot,
                i as f32,
                spacing_x,
                spacing_z,
            );

            let card = &mut self.opp_2_melded_cards[i];
            card.set_target(new_target_pos, target_rot);
        }
    }

    fn rearrange_opp_3_melded_cards(&mut self) {
        if self.opp_3_melded_cards.is_empty() {
            return;
        }

        let spacing_x = 0.1;
        let spacing_z = 0.002;
        let count = self.opp_3_melded_cards.len() as f32;

        let start_x = ((count - 1.0) * spacing_x) / 2.0;
        let base_target_pos = vec3(1.5, 0., start_x);
        let target_rot = vec3(-90., 90., 0.0);

        self.opp_3_melded_cards[0].set_target(base_target_pos, target_rot);

        for i in 1..self.opp_3_melded_cards.len() {
            let new_target_pos = CardElement::get_indexed_position(
                base_target_pos,
                target_rot,
                i as f32,
                spacing_x,
                spacing_z,
            );

            let card = &mut self.opp_3_melded_cards[i];
            card.set_target(new_target_pos, target_rot);
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
        let target_rot = vec3(0.0, 180.0, 0.0);

        self.opp_1_hand[0].set_target_with_dim(base_target_pos, target_rot, 0.8);

        for i in 1..self.opp_1_hand.len() {
            let new_target_pos = CardElement::get_indexed_position(
                base_target_pos,
                target_rot,
                i as f32,
                -spacing_x,
                spacing_z,
            );

            let card = &mut self.opp_1_hand[i];
            card.set_target_with_dim(new_target_pos, target_rot, 0.8);
        }
    }

    fn rearrange_opp_2_hand(&mut self) {
        if self.opp_2_hand.is_empty() {
            return;
        }

        let spacing_x = 0.16;
        let spacing_z = 0.002;
        let count = self.opp_2_hand.len() as f32;

        let start_x = ((count - 1.0) * -spacing_x) / 2.0;
        let base_target_pos = vec3(start_x, 0.8 / 2., 2.5);

        self.opp_2_hand[0].set_target_with_dim(base_target_pos, vec3(0.0, 0., 0.0), 0.8);

        for i in 1..self.opp_2_hand.len() {
            let new_target_pos = CardElement::get_indexed_position(
                base_target_pos,
                self.opp_2_hand[0].rotation,
                i as f32,
                spacing_x,
                -spacing_z,
            );

            let card = &mut self.opp_2_hand[i];
            card.set_target_with_dim(new_target_pos, vec3(0.0, 0., 0.0), 0.8);
        }
    }

    fn rearrange_opp_3_hand(&mut self) {
        if self.opp_3_hand.is_empty() {
            return;
        }

        let spacing_x = 0.16;
        let spacing_z = 0.002;
        let count = self.opp_3_hand.len() as f32;

        let start_x = ((count - 1.0) * spacing_x) / 2.0;
        let base_target_pos = vec3(2.5, 0.8 / 2., start_x);
        let target_rot = vec3(0.0, 90., 0.0);

        self.opp_3_hand[0].set_target_with_dim(base_target_pos, target_rot, 0.8);

        for i in 1..self.opp_3_hand.len() {
            let new_target_pos = CardElement::get_indexed_position(
                base_target_pos,
                target_rot,
                i as f32,
                spacing_x,
                -spacing_z,
            );

            let card = &mut self.opp_3_hand[i];
            card.set_target_with_dim(new_target_pos, target_rot, 0.8);
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
        let mut base_target_pos = vec3(-1.8, 0.8 / 2., start_z);
        let x_degree = self.calculate_look_at_cam_tilt(base_target_pos) + 180.;
        let target_rot = vec3(x_degree, 90.0, 0.0);

        if let Some(ind) = self.selected_card_index {
            if ind == 0 {
                base_target_pos.y = 0.7;
            }
        } else if self.selected_cards_for_meld.contains(&0) {
            base_target_pos.y = 0.7;
        }

        self.hand[0].set_target_with_dim(base_target_pos, target_rot, 0.8);

        let base_target_pos = vec3(-1.8, 0.8 / 2., start_z);

        for i in 1..self.hand.len() {
            let mut new_target_pos = CardElement::get_indexed_position(
                base_target_pos,
                target_rot,
                i as f32,
                spacing_x,
                spacing_z,
            );

            if let Some(ind) = self.selected_card_index {
                if ind == i {
                    new_target_pos.y = 0.7;
                }
            } else if self.selected_cards_for_meld.contains(&i) {
                new_target_pos.y = 0.7;
            }

            let card = &mut self.hand[i];

            card.set_target_with_dim(new_target_pos, target_rot, 0.8);
        }
    }

    fn rearrange_hand_side(&mut self) {
        if self.hand.is_empty() {
            return;
        }

        let spacing_x = 0.1;
        let spacing_z = 0.002;

        let base_target_pos = vec3(-1.5, 0.8 / 2., 2.);
        let x_degree = self.calculate_look_at_cam_tilt(base_target_pos);

        self.hand[0].set_target(base_target_pos, vec3(x_degree, 90.0, 0.0));

        for i in 1..self.hand.len() {
            let new_target_pos = CardElement::get_indexed_position(
                base_target_pos,
                self.hand[0].rotation,
                i as f32,
                spacing_x,
                spacing_z,
            );

            let x_degree = self.calculate_look_at_cam_tilt(new_target_pos);

            let card = &mut self.hand[i];
            card.set_target(new_target_pos, vec3(x_degree, 90.0, 0.0));
        }
    }

    fn discard_from_hand(&mut self, card_type: Card) {
        let mut index: Option<usize> = None;
        for i in 0..self.hand.len() {
            if let Some(card_element) = &self.hand[i].card {
                if card_element == &card_type {
                    index = Some(i);
                    break;
                }
            }
        }

        if index.is_none() {
            return;
        }

        if self.hand.len() - 1 >= index.unwrap() {
            let mut card = self.hand.remove(index.unwrap());
            let rand_rot_y: f32 = rand::gen_range(90. - 7.5, 90.0 + 15.);
            let rand_x: f32 = rand::gen_range(-0.125, 0.125);
            let rand_z: f32 = rand::gen_range(-0.125, 0.125);

            card.set_target_with_dim(
                vec3(rand_x, self.discard_pile.len() as f32 * 0.001, rand_z),
                vec3(270.0, rand_rot_y, 0.0),
                1.2,
            );
            self.discard_pile.push(card);
            self.rearrange_hand();
        }
    }

    fn discard_from_opp_1_hand(&mut self, card_type: Card) {
        if let Some(mut card) = self.opp_1_hand.pop() {
            let rand_rot_y: f32 = rand::gen_range(0.0, 360.0);
            let rand_x: f32 = rand::gen_range(-0.125, 0.125);
            let rand_z: f32 = rand::gen_range(-0.125, 0.125);

            card.set_target_with_dim(
                vec3(rand_x, self.discard_pile.len() as f32 * 0.001, rand_z),
                vec3(-90.0, rand_rot_y, 0.0),
                1.2,
            );
            card.set_card(Some(&card_type));
            self.discard_pile.push(card);
            self.rearrange_opp_1_hand();
        }
    }

    fn discard_from_opp_2_hand(&mut self, card_type: Card) {
        if let Some(mut card) = self.opp_2_hand.pop() {
            let rand_rot_y: f32 = rand::gen_range(0.0, 360.0);
            let rand_x: f32 = rand::gen_range(-0.125, 0.125);
            let rand_z: f32 = rand::gen_range(-0.125, 0.125);

            card.set_target_with_dim(
                vec3(rand_x, self.discard_pile.len() as f32 * 0.001, rand_z),
                vec3(-90.0, rand_rot_y, 0.0),
                1.2,
            );
            card.set_card(Some(&card_type));
            self.discard_pile.push(card);
            self.rearrange_opp_2_hand();
        }
    }

    fn discard_from_opp_3_hand(&mut self, card_type: Card) {
        if let Some(mut card) = self.opp_3_hand.pop() {
            let rand_rot_y: f32 = rand::gen_range(0.0, 360.0);
            let rand_x: f32 = rand::gen_range(-0.125, 0.125);
            let rand_z: f32 = rand::gen_range(-0.125, 0.125);

            card.set_target_with_dim(
                vec3(rand_x, self.discard_pile.len() as f32 * 0.001, rand_z),
                vec3(-90.0, rand_rot_y, 0.0),
                1.2,
            );
            card.set_card(Some(&card_type));
            self.discard_pile.push(card);
            self.rearrange_opp_3_hand();
        }
    }

    fn pop_from_stock_pile(&mut self) -> Option<CardElement> {
        if let Some(card) = self.stock_pile.pop() {
            return Some(card);
        }

        return None;
    }

    fn pop_from_discard_pile(&mut self) -> Option<CardElement> {
        if let Some(card) = self.discard_pile.pop() {
            return Some(card);
        }

        return None;
    }

    fn draw_for_hand(&mut self, card: CardElement) {
        self.hand.push(card);

        self.rearrange_hand();
    }

    fn draw_for_opp_1(&mut self, card: CardElement) {
        self.opp_1_hand.push(card);

        self.rearrange_opp_1_hand();
    }

    fn draw_for_opp_2(&mut self, card: CardElement) {
        self.opp_2_hand.push(card);

        self.rearrange_opp_2_hand();
    }

    fn draw_for_opp_3(&mut self, card: CardElement) {
        self.opp_3_hand.push(card);

        self.rearrange_opp_3_hand();
    }

    pub fn calculate_look_at_cam_tilt(&self, card_pos: Vec3) -> f32 {
        let delta_x = (self.camera.position.x - self.camera.position.y).abs();
        let delta_y = self.camera.position.y - card_pos.y;

        let tilt_angle_deg = f32::atan2(delta_x, delta_y).to_degrees();

        tilt_angle_deg
    }

    pub fn handle_game_event(&mut self, event: GameEvent) {
        match event {
            GameEvent::CurrentTurn(player_id) => {
                let placement = self.get_placement(player_id).unwrap();
                let current_arrow_angle = self.turn_arrow.target_rot.y % 360.;
                let minus_angle: f32;
                match placement {
                    PlayerPlacement::Left => {
                        minus_angle = 360. - current_arrow_angle;
                    }
                    PlayerPlacement::Front => {
                        minus_angle = 270. - current_arrow_angle;
                    }
                    PlayerPlacement::Right => {
                        minus_angle = 180. - current_arrow_angle;
                    }
                    PlayerPlacement::Me => {
                        minus_angle = 90. - current_arrow_angle;
                    }
                }

                self.turn_arrow
                    .set_target(self.turn_arrow.target_rot + vec3(0., minus_angle, 0.));
            }
            GameEvent::Meld { player_id, cards } => {
                let placement = self.get_placement(player_id).unwrap();

                match placement {
                    PlayerPlacement::Left => {
                        self.meld_opp_1_cards(cards);
                    }

                    PlayerPlacement::Front => {
                        self.meld_opp_3_cards(cards);
                    }

                    PlayerPlacement::Right => {
                        self.meld_opp_2_cards(cards);
                    }

                    PlayerPlacement::Me => {
                        self.meld();
                    }
                }
            }
            GameEvent::Turn(turn) => match turn {
                TurnEvent::Discard { player_id, card } => {
                    let placement = self.get_placement(player_id).unwrap();
                    match placement {
                        PlayerPlacement::Left => {
                            self.discard_from_opp_1_hand(card);
                        }
                        PlayerPlacement::Front => {
                            self.discard_from_opp_3_hand(card);
                        }
                        PlayerPlacement::Right => {
                            self.discard_from_opp_2_hand(card);
                        }
                        PlayerPlacement::Me => {
                            self.discard_from_hand(card);
                        }
                    }
                }
                TurnEvent::Draw { player_id, source } => {
                    let placement = self.get_placement(player_id).unwrap();
                    let card: Option<CardElement>;

                    match source {
                        DrawSource::StockPile => match placement {
                            PlayerPlacement::Me => {
                                card = None;
                            }
                            _ => {
                                card = self.pop_from_stock_pile();
                            }
                        },
                        DrawSource::DiscardPile => {
                            card = self.pop_from_discard_pile();
                        }
                    }

                    if let Some(card_element) = card {
                        match placement {
                            PlayerPlacement::Left => {
                                self.draw_for_opp_1(card_element);
                            }
                            PlayerPlacement::Front => {
                                self.draw_for_opp_3(card_element);
                            }
                            PlayerPlacement::Right => {
                                self.draw_for_opp_2(card_element);
                            }

                            // a card drawn by a player will received via GameEvent::DrawnCard(Card)
                            // since they need to know the card type
                            PlayerPlacement::Me => {
                                if let DrawSource::DiscardPile = source {
                                    self.draw_for_hand(card_element);
                                }
                            }
                        }
                    }
                }
            },
            GameEvent::PlayersTurn(arr) => {
                for i in 0..arr.len() {
                    if arr[i] == self.self_id {
                        self.self_index = i;
                        break;
                    }
                }
                self.player_turns = arr;
            }

            GameEvent::SelfCard { cards, stock_number, each_hand } => {
                self.is_sharing_card =
                    self.get_placement(self.player_turns[self.current_index_share]);
                self.self_cards = cards;
                self.stock_number = stock_number;
                self.each_hand = each_hand;

            }
            GameEvent::DrawnCard(card) => {
                let card_option = self.pop_from_stock_pile();
                if let Some(mut card_element) = card_option {
                    card_element.set_card(Some(&card));
                    self.draw_for_hand(card_element);
                }
            }
            GameEvent::PlayersHands(ps) => {
                for (pid, hand) in ps {
                    if let Some(p) = self.get_placement(pid) {
                        match p {
                            PlayerPlacement::Right => {
                                self.drop_opp_2_hand(hand);
                            }
                            PlayerPlacement::Front => {
                                self.drop_opp_3_hand(hand);
                            }
                            PlayerPlacement::Left => {
                                self.drop_opp_1_hand(hand);
                            }

                            _ => {}
                        }
                    }
                }
            }
        }
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

        self.turn_arrow.draw();

        for i in &self.opp_1_hand {
            i.draw();
        }

        for i in &self.opp_2_hand {
            i.draw();
        }

        for i in &self.opp_3_hand {
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
            i.draw();
        }

        for i in &self.opp_1_melded_cards {
            i.draw();
        }

        for i in &self.opp_2_melded_cards {
            i.draw();
        }

        for i in &self.opp_3_melded_cards {
            i.draw();
        }

        let (mx, my) = mouse_position();

        set_default_camera();
        draw_text("WELCOME TO 3D WORLD", 10.0, 20.0, 30.0, BLACK);
        draw_text(format!("X: {}, Y: {}", mx, my), 10.0, 40.0, 30.0, BLACK);
        if self.is_melding {
            draw_text("Is Melding", 10.0, 60.0, 30.0, BLACK);
        }
    }

    fn update(&mut self, state: &Option<State>) -> Option<State> {
        for i in &mut self.objects {
            if let Some(state) = i.update(ParentState::root(), state) {
                return Some(state);
            }
        }

        if let Some(a) = &self.is_sharing_card {
            self.elapsed_time += get_frame_time();

            if self.elapsed_time >= self.target_time {
                match a {
                    PlayerPlacement::Me => {
                        if let Some(card) = self.self_cards.pop() {
                            let mut card_element = self.pop_from_stock_pile().unwrap();
                            card_element.set_card(Some(&card));
                            self.draw_for_hand(card_element);
                        }
                    }
                    PlayerPlacement::Right => {
                        let card_element = self.pop_from_stock_pile().unwrap();
                        self.draw_for_opp_2(card_element);
                    }
                    PlayerPlacement::Front => {
                        let card_element = self.pop_from_stock_pile().unwrap();
                        self.draw_for_opp_3(card_element);
                    }
                    PlayerPlacement::Left => {
                        let card_element = self.pop_from_stock_pile().unwrap();
                        self.draw_for_opp_1(card_element);
                    }
                }

                self.current_index_share += 1;

                self.is_sharing_card = self.get_placement(
                    self.player_turns[(self.current_index_share) % self.player_turns.len()],
                );

                self.shared_cards += 1;

                if self.shared_cards as usize >= self.player_turns.len() * self.each_hand {
                    self.is_sharing_card = None;
                }

                self.target_time += 0.3;
            }
        }

        self.update_stock_pile_hover();
        self.turn_arrow.update();

        for i in &mut self.opp_1_hand {
            i.update();
        }

        for i in &mut self.opp_2_hand {
            i.update();
        }

        for i in &mut self.opp_3_hand {
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
            i.update();
        }

        for i in &mut self.opp_1_melded_cards {
            i.update();
        }

        for i in &mut self.opp_2_melded_cards {
            i.update();
        }

        for i in &mut self.opp_3_melded_cards {
            i.update();
        }

        if is_key_released(KeyCode::O) {
            self.is_melding = !self.is_melding;
            self.selected_cards_for_meld.clear();
            self.selected_card_index = None;

            self.rearrange_hand();
        }

        self.update_card_interaction();

        let (mx, my) = mouse_position();

        if self.is_melding {
            if is_key_released(KeyCode::P) {
                let mut cards: Vec<Card> = Vec::new();
                for i in &self.selected_cards_for_meld {
                    let card = self.hand[*i].card;
                    cards.push(card.unwrap());
                }

                self.is_melding = false;
                return Some(State::InGameCommand(GameCommand::Meld { cards: cards }));
            }

            if is_mouse_button_released(MouseButton::Left) {
                if let Some(index) = self.hovered_card_index {
                    if self.selected_cards_for_meld.contains(&index) {
                        let mut index_to_remove: usize = 0;
                        for (i, val) in &mut self.selected_cards_for_meld.iter().enumerate() {
                            if *val == index {
                                index_to_remove = i;
                            }
                        }
                        self.selected_cards_for_meld.remove(index_to_remove);
                        self.rearrange_hand();
                    } else {
                        self.selected_cards_for_meld.push(index);
                        self.rearrange_hand();
                    }
                } else {
                    self.selected_cards_for_meld.clear();
                    self.rearrange_hand();
                }
            }
        } else {
            if is_mouse_button_released(MouseButton::Left) {
                if let Some(index) = self.hovered_card_index {
                    //
                    // Swap cards
                    //
                    //
                    if let Some(prev_card_index) = self.selected_card_index {
                        self.hand.swap(index, prev_card_index);
                        self.selected_card_index = None;
                        self.rearrange_hand();
                    } else {
                        self.rearrange_hand();
                        self.selected_card_index = Some(index);
                        self.rearrange_hand();
                    }
                } else {
                    if mx >= screen_width() / 2. - 100.
                        && mx <= screen_width() / 2. + 100.0
                        && my >= screen_height() / 2. - 200.
                        && my <= screen_height() / 2. + 50.
                    {
                        // Discard
                        //
                        if let Some(index) = self.selected_card_index {
                            // Here we will send turn command via WebSocket to discard
                            // self.discard_from_hand() will be called after the response from server
                            //
                            // self.discard_from_hand(index);

                            let selected_card = &self.hand[index];

                            self.selected_card_index = None;

                            return Some(State::InGameCommand(GameCommand::Turn(
                                TurnCommand::Discard(selected_card.card.unwrap().clone()),
                            )));
                        } else {
                            // Try to draw from discard pile
                            return Some(State::InGameCommand(GameCommand::Turn(
                                TurnCommand::Draw(DrawSource::DiscardPile),
                            )));
                        }
                    } else {
                        self.selected_card_index = None;
                        self.rearrange_hand();
                    }

                    if self.is_hover_stock_pile {
                        return Some(State::InGameCommand(GameCommand::Turn(TurnCommand::Draw(
                            DrawSource::StockPile,
                        ))));
                    }
                }
            }
        }

        return None;
    }
}
