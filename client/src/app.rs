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
        }
    }

    pub async fn init(&mut self) {
        self.game_state = GameState::Loading(Loading::Initialization);

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
                        self.current_page =
                            Some(Box::new(MainMenu::new(self.font.clone())));
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
                            self.current_page = Some(Box::new(Room::new(
                                ws,
                                room_id.clone(),
                                self.player_id.unwrap(),
                            ).load_ui(self.font.clone())))
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
                            self.current_page = Some(Box::new(Room::new(
                                ws,
                                response,
                                self.player_id.unwrap(),
                            ).load_ui(self.font.clone())))
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
                    let req = RequestBuilder::new("http://localhost:6767/room/create")
                        .method(Method::Post)
                        .header("Cookie", &format!("id={}", self.player_id.unwrap()))
                        .header("Content-Type", "application/json")
                        .body(r#"{"allow_court_stacking": true,"free_hit": true,"allow_railing": true,"with_joker": true,"hitter_scoring": true,"number_of_jokers": "None","joker_type": null,"allow_closing": false}"#)
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
