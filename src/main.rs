use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
enum CourtType {
    Jack,
    Queen,
    King
}

#[derive(Debug, EnumIter, Clone, Copy)]
enum JokerType {
    Red,
    Black
}

#[derive(Debug, EnumIter, Clone, Copy)]
enum CardIcon {
    Heart,
    Diamond,
    Spade,
    Club
}

#[derive(Debug)]
enum CardType {
    Ace,
    Court(CourtType),
    Spot(SpotNumber),
    Joker(JokerType)
}

#[derive(Debug, EnumIter)]
enum SpotNumber {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten
}

#[derive(Debug)]
struct Card {
    card_icon: Option<CardIcon>,
    card_type: CardType,
}

impl Card {
    pub fn generate_full_deck() -> Vec<Card> {
        let mut deck: Vec<Card> = Vec::new();

        for i in CardIcon::iter() {
            deck.push(Card{ card_icon: Some(i.clone()), card_type: CardType::Ace });

            for court_type in CourtType::iter() {
                deck.push(Card { card_icon: Some(i.clone()), card_type: CardType::Court(court_type) });
            }

            for spot_number in SpotNumber::iter() {
                deck.push(Card { card_icon: Some(i.clone()), card_type: CardType::Spot(spot_number) });
            }
        }

        for joker_type in JokerType::iter() {
            deck.push(Card { card_icon: None, card_type: CardType::Joker(joker_type.clone()) });
        }

        return deck;
    }
    fn get_card_power(&self) -> u32 {
        match &self.card_type {
            CardType::Joker(_) => {
                return 25;
            },
            CardType::Ace => {
                return 15;
            },
            CardType::Court(n) => {
                return 10;
            },
            CardType::Spot(n) => {
                return 5;
            }
        }
    }
}

struct Player {
    uname: String,
}

struct GamePlayer {
    player: Player,
}

struct CardGame {
    players: Vec<GamePlayer>,
    kartu_guakan: Vec<Card>,
    bookie: Player,
    config: CardGameConfig
}

impl CardGame {
    fn start_game(cfg: CardGameConfig) {
        

    }
}

struct CardGameConfig {
    players: Vec<GamePlayer>,
    boleh_timpa_jqk: bool,
    tutukan_bebas: bool,
    boleh_ngerail: bool,
    with_joker: bool,
    joker_type: Option<Card>
}

impl CardGameConfig {
    pub fn create(
        players: Vec<Player>,
        boleh_timpa_jqk: bool, 
        tutukan_bebas: bool, boleh_ngerail:bool, 
        with_joker: bool,
        joker_type: Option<Card>
    ) -> Result<CardGameConfig, String> {

        let player_count = players.len();

        if player_count < 3 || player_count > 4 {
            return Err(String::from("Jumlah player antara 3 atau 4"));
        }

        let mut cfg = Self { 
            players: vec![], 
            boleh_timpa_jqk: boleh_timpa_jqk, 
            tutukan_bebas: tutukan_bebas, 
            boleh_ngerail: boleh_ngerail, 
            with_joker: with_joker,
            joker_type: None, 
        };

        for i in players {
            cfg.players.push(GamePlayer { player: i });
        }

        match joker_type {
            Some(n) => {
                match n.card_type {
                    CardType::Spot(_) => {
                        cfg.joker_type = Some(n);
                    },
                    _ => {
                        return Err(String::from("Tipe joker harus angka biasa"))
                    }
                }
            }
            None => {}
        }

        Ok(cfg)
    }
}



fn main() {
    let deck = Card::generate_full_deck();
    println!("{:#?}", deck);
    println!("{}", deck.len());
}