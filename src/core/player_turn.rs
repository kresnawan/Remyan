use crate::core::{card::Card, protocol::DrawSource};



#[derive(Debug)]
pub struct PlayerTurn {
    pub index: usize,
    pub draw_source: Option<DrawSource>,
    pub drawn_card: Option<Vec<Card>>,
    pub melded_card: Option<Vec<Card>>,
    pub discarded_card: Option<Card>,
}

impl PlayerTurn {
    pub fn new() -> Self {
        PlayerTurn {
            index: 0,
            draw_source: None,
            drawn_card: None,
            melded_card: None,
            discarded_card: None,
        }
    }

    pub fn is_complete(&self) -> bool {
        match self.draw_source {
            Some(DrawSource::DiscardPile(_)) => {
                if let Some(_) = self.drawn_card
                    && let Some(_) = self.melded_card
                    && let Some(_) = self.discarded_card
                {
                    return true;
                }
            }
            Some(DrawSource::StockPile) => {
                if let Some(_) = self.drawn_card
                    && let Some(_) = self.discarded_card
                {
                    return true;
                }
            }
            None => {
                return false;
            }
        }

        return false;
    }

    pub fn reset(&mut self) {
        self.draw_source = None;
        self.drawn_card = None;
        self.melded_card = None;
        self.discarded_card = None;
    }
}
