use std::{process::exit, sync::Arc};

use macroquad::{
    color::GREEN,
    window::{clear_background, next_frame},
};
use quad_net::{
    http_request::{Method, Request, RequestBuilder},
    web_socket::WebSocket,
};

use crate::{
    page::{Page, Pages, main_menu::MainMenu, room::Room},
    state::State,
    ui::config::font::Nunito,
};

pub enum Loading {
    JoinRoom(String),
    CreateRoom,
    Initialization,
    LeaveRoom,
}

pub enum GameState {
    Loading(Loading),
    Running,
    Uninitialized,
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
        }
    }

    pub async fn init(&mut self) {
        let mut global_state: Option<State> = None;
        self.game_state = GameState::Loading(Loading::Initialization);

        let req = RequestBuilder::new("http://localhost:6767/auth/id")
            .method(Method::Get)
            .send();

        self.get_id_request = Some(req);

        loop {
            match &self.game_state {
                GameState::Loading(typ) => {
                    clear_background(GREEN);

                    match typ {
                        Loading::Initialization => {
                            let req = self.get_id_request.as_mut().unwrap();
                            if let Some(v) = req.try_recv() {
                                match v {
                                    Ok(id) => {
                                        let parsed: u32 = id.parse().unwrap();

                                        println!("{}", parsed);
                                        self.player_id = Some(parsed);
                                        self.current_page = Some(Box::new(MainMenu::new(
                                            "Kresna",
                                            self.font.clone(),
                                        )));
                                        self.game_state = GameState::Running;
                                    }

                                    Err(err) => {
                                        println!("{:#?}", err);
                                        exit(1);
                                    }
                                }
                            }
                        }

                        Loading::JoinRoom(room_id) => {
                            if let Some(req) = &mut self.join_room_request {
                                if let Some(value) = req.try_recv() {
                                    match value {
                                        Ok(response) => {
                                            println!("{}", response);
                                            match WebSocket::connect(String::from(format!(
                                                "ws://localhost:6767/ws/connect?room_id={}&player_id={}",
                                                room_id,
                                                self.player_id.unwrap()
                                            ))) {
                                                Ok(ws) => {
                                                    self.current_page = Some(Box::new(Room::new(
                                                        self.font.clone(),
                                                        ws,
                                                        room_id.clone(),
                                                        self.player_id.unwrap(),
                                                    )))
                                                }
                                                Err(err) => {
                                                    println!("{:#?}", err);
                                                }
                                            }
                                        }
                                        Err(http_error) => {
                                            println!("{}", http_error)
                                        }
                                    }

                                    self.game_state = GameState::Running;
                                }
                            }
                        }

                        Loading::CreateRoom => {
                            if let Some(req) = &mut self.create_room_request {
                                if let Some(value) = req.try_recv() {
                                    match value {
                                        Ok(response) => {
                                            println!("{}", response);
                                            if let Ok(ws) = WebSocket::connect(String::from(
                                                format!(
                                                    "ws://localhost:6767/ws/connect?room_id={}&player_id={}",
                                                    response.clone(),
                                                    self.player_id.unwrap()
                                                ),
                                            )) {
                                                self.current_page = Some(Box::new(Room::new(
                                                    self.font.clone(),
                                                    ws,
                                                    response,
                                                    self.player_id.unwrap()
                                                )))
                                            }
                                        }
                                        Err(http_error) => {
                                            println!("{}", http_error.to_string())
                                        }
                                    }

                                    self.create_room_request = None;
                                    self.game_state = GameState::Running;
                                }
                            }
                        }

                        Loading::LeaveRoom => {
                            self.current_page =
                                Some(Box::new(MainMenu::new("Kres", self.font.clone())));
                            self.game_state = GameState::Running;
                        }
                        _ => {}
                    }
                }

                GameState::Running => {
                    if let Some(state) = self.current_page.as_mut().unwrap().update(&global_state) {
                        match state {
                            State::CreateRoom => {
                                let req = RequestBuilder::new("http://localhost:6767/room/create")
                                .method(Method::Post)
                                .header("Cookie", &format!("id={}", self.player_id.unwrap()))
                                .header("Content-Type", "application/json")
                                .body(r#"{"allow_court_stacking": true,"free_hit": true,"allow_railing": true,"with_joker": true,"hitter_scoring": true,"number_of_jokers": "Two","joker_type": null}"#)
                                .send();
                                self.create_room_request = Some(req);
                                self.game_state = GameState::Loading(Loading::CreateRoom);
                                global_state = None;
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
                                global_state = None;
                            }

                            State::LeaveRoom => {
                                self.game_state = GameState::Loading(Loading::LeaveRoom);
                                global_state = None;
                            }

                            _ => global_state = Some(state),
                        }
                    }
                    self.current_page.as_ref().unwrap().draw();
                }

                _ => {}
            }

            next_frame().await
        }
    }
}
