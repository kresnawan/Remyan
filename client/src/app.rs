use std::{collections::HashMap, process::exit, sync::Arc};

use macroquad::{
    color::GREEN,
    texture::{Texture2D, load_texture},
    window::{clear_background, next_frame},
};
use quad_net::{
    http_request::{Method, Request, RequestBuilder},
    web_socket::WebSocket,
};
use remyan_core::{CardIcon, CardType, CourtType, Deck, JokerType, RoomConfig, SpotNumber};

use crate::{
    page::{Page, main_menu::MainMenu, room::Room},
    state::State,
    ui::config::font::Nunito,
};

#[derive(Clone)]
pub enum Loading {
    JoinRoom(String),
    CreateRoom,
    Initialization,
    LeaveRoom,
}

#[derive(Clone)]
pub enum GameState {
    Loading(Loading),
    Running,
    Uninitialized,
}

#[derive(Debug)]
pub struct CardTextures {
    pub cards: HashMap<remyan_core::Card, Texture2D>,
    pub empty_texture: Texture2D,
    pub back_texture: Texture2D,
    pub arrow_texture: Texture2D,
}

impl CardTextures {
    pub async fn load() -> CardTextures {
        let mut cards: HashMap<remyan_core::Card, Texture2D> = HashMap::new();
        let mut deck = Deck::new(true);
        while let Some(card) = deck.cards.pop() {
            let mut file_name = String::new();
            if let Some(icon) = card.card_icon {
                match icon {
                    CardIcon::Club => {
                        file_name.push_str("club");
                    }
                    CardIcon::Diamond => {
                        file_name.push_str("diamond");
                    }
                    CardIcon::Spade => {
                        file_name.push_str("spades");
                    }
                    CardIcon::Heart => {
                        file_name.push_str("heart");
                    }
                }

                file_name.push_str("-");

                match card.card_type {
                    CardType::Ace => {
                        file_name.push_str("01");
                    }
                    CardType::Spot(number) => match number {
                        SpotNumber::Two => file_name.push_str("02"),
                        SpotNumber::Three => file_name.push_str("03"),
                        SpotNumber::Four => file_name.push_str("04"),
                        SpotNumber::Five => file_name.push_str("05"),
                        SpotNumber::Six => file_name.push_str("06"),
                        SpotNumber::Seven => file_name.push_str("07"),
                        SpotNumber::Eight => file_name.push_str("08"),
                        SpotNumber::Nine => file_name.push_str("09"),
                        SpotNumber::Ten => file_name.push_str("10"),
                    },

                    CardType::Court(court) => match court {
                        CourtType::Jack => file_name.push_str("11"),
                        CourtType::Queen => file_name.push_str("12"),
                        CourtType::King => file_name.push_str("13"),
                    },

                    _ => {}
                }
            } else {
                match card.card_type {
                    CardType::Joker(joker) => match joker {
                        JokerType::Black => file_name.push_str("joker-black"),
                        JokerType::Red => file_name.push_str("joker-red"),
                    },

                    _ => {}
                }
            }

            file_name.push_str(".png");

            let texture = load_texture(&format!("assets/card/{}", file_name))
                .await
                .unwrap();
            cards.insert(card, texture);
        }

        let empty_texture = load_texture("assets/card/card-empty.png").await.unwrap();
        let back_texture = load_texture("assets/card/card-back.png").await.unwrap();
        let arrow_texture = load_texture("assets/arrow-01.png").await.unwrap();

        CardTextures {
            cards,
            empty_texture,
            back_texture,
            arrow_texture
        }
    }

    pub fn get(&self, card: &remyan_core::Card) -> Texture2D {
        let texture = self.cards.get(card).unwrap().clone();
        texture
    }

    pub fn get_empty_texture(&self) -> Texture2D {
        self.empty_texture.clone()
    }

    pub fn get_back_texture(&self) -> Texture2D {
        self.back_texture.clone()
    }
}

pub struct App {
    pub current_page: Option<Box<dyn Page>>,
    pub game_state: GameState,
    pub player_id: Option<u32>,
    pub pre_allocated_pages: Vec<Box<dyn Page>>,
    pub font: Arc<Nunito>,
    pub join_room_request: Option<Request>,
    pub create_room_request: Option<Request>,
    pub get_id_request: Option<Request>,
    pub global_state: Option<State>,
    pub card_back_texture: Texture2D,
    pub card_textures: Option<Arc<CardTextures>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_page: None,
            game_state: GameState::Uninitialized,
            player_id: None,
            pre_allocated_pages: Vec::new(),
            font: Arc::new(Nunito::load()),
            join_room_request: None,
            create_room_request: None,
            get_id_request: None,
            global_state: None,
            card_back_texture: Texture2D::empty(),
            card_textures: None,
        }
    }

    pub async fn init(&mut self) {
        self.game_state = GameState::Loading(Loading::Initialization);

        let card_back_texture = load_texture("assets/card/card-back.png").await.unwrap();
        self.card_back_texture = card_back_texture;

        let card_textures = Arc::new(CardTextures::load().await);

        self.card_textures = Some(card_textures);

        let get_id_req = RequestBuilder::new("http://localhost:6767/auth/id")
            .method(Method::Get)
            .send();

        self.get_id_request = Some(get_id_req);

        loop {
            if let GameState::Loading(typ) = self.game_state.clone() {
                clear_background(GREEN);
                self.handle_loading_state(&typ).await;
            } else {
                self.handle_running_state().await;
            }

            next_frame().await
        }
    }

    async fn handle_loading_state(&mut self, typ: &Loading) {
        match typ {
            Loading::Initialization => {
                let req = self.get_id_request.as_mut().unwrap();
                let Some(v) = req.try_recv() else {
                    return;
                };

                match v {
                    Ok(id) => {
                        let parsed: u32 = id.parse().unwrap();

                        println!("{}", parsed);
                        self.player_id = Some(parsed);
                        self.current_page = Some(Box::new(MainMenu::new(self.font.clone())));
                        self.game_state = GameState::Running;
                    }

                    Err(err) => {
                        println!("{:#?}", err);
                        exit(1);
                    }
                }
            }

            Loading::JoinRoom(room_id) => {
                let Some(req) = &mut self.join_room_request else {
                    return;
                };

                let Some(value) = req.try_recv() else {
                    return;
                };

                if let Ok(response) = &value {
                    println!("{}", response);
                    match WebSocket::connect(format!(
                        "ws://localhost:6767/ws/connect?room_id={}&player_id={}",
                        room_id,
                        self.player_id.unwrap()
                    )) {
                        Ok(ws) => {
                            self.current_page = Some(Box::new(
                                Room::new(
                                    ws,
                                    room_id.clone(),
                                    self.player_id.unwrap(),
                                    self.card_textures.as_ref().unwrap().clone()
                                )
                                .load_ui(self.font.clone()),
                            ))
                        }
                        Err(err) => {
                            println!("{:#?}", err);
                        }
                    }
                }

                if let Err(err) = &value {
                    println!("{}", err);
                }

                self.game_state = GameState::Running;
            }

            Loading::CreateRoom => {
                let Some(req) = &mut self.create_room_request else {
                    return;
                };
                let Some(value) = req.try_recv() else {
                    return;
                };

                match value {
                    Ok(response) => {
                        println!("{}", response);
                        if let Ok(ws) = WebSocket::connect(String::from(format!(
                            "ws://localhost:6767/ws/connect?room_id={}&player_id={}",
                            response.clone(),
                            self.player_id.unwrap()
                        ))) {
                            self.current_page = Some(Box::new(
                                Room::new(
                                    ws,
                                    response,
                                    self.player_id.unwrap(),
                                    self.card_textures.as_ref().unwrap().clone()
                                )
                                .load_ui(self.font.clone()),
                            ))
                        }
                    }
                    Err(http_error) => {
                        println!("{}", http_error.to_string())
                    }
                }

                self.create_room_request = None;
                self.game_state = GameState::Running;
            }

            Loading::LeaveRoom => {
                self.current_page = Some(Box::new(MainMenu::new(self.font.clone())));
                self.game_state = GameState::Running;
            }
        }
    }

    async fn handle_running_state(&mut self) {
        if let Some(state) = self
            .current_page
            .as_mut()
            .unwrap()
            .update(&self.global_state)
        {
            match state {
                State::CreateRoom => {
                    let room_config = RoomConfig::default();
                    let room_config_str = serde_json::to_string(&room_config).unwrap();
                    let req = RequestBuilder::new("http://localhost:6767/room/create")
                        .method(Method::Post)
                        // .header("Cookie", &format!("id={}", self.player_id.unwrap()))
                        .header("Content-Type", "application/json")
                        .body(&room_config_str)
                        .send();
                    self.create_room_request = Some(req);
                    self.game_state = GameState::Loading(Loading::CreateRoom);
                    self.global_state = None;
                }

                State::JoinRoom(room_id) => {
                    let req = RequestBuilder::new(&format!(
                        "http://localhost:6767/room/join?room_id={}",
                        room_id
                    ))
                    .header("Cookie", &format!("id={}", self.player_id.unwrap()))
                    .method(Method::Post)
                    .send();
                    self.join_room_request = Some(req);

                    self.game_state = GameState::Loading(Loading::JoinRoom(room_id));
                    self.global_state = None;
                }

                State::LeaveRoom => {
                    self.game_state = GameState::Loading(Loading::LeaveRoom);
                    self.global_state = None;
                }

                State::Reset => {
                    self.global_state = None;
                }

                _ => self.global_state = Some(state),
            }
        }
        self.current_page.as_ref().unwrap().draw();
    }
}
