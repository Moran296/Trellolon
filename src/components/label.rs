use super::component::Component;
use super::component::Creatable;
use crate::client::Client;
use crate::Board;
use crate::Card;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LabelColor {
    Yellow,
    Purple,
    Blue,
    Red,
    Green,
    Orange,
    Black,
    Sky,
    Pink,
    Lime,
}

impl fmt::Display for LabelColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LabelColor::Yellow => write!(f, "yellow"),
            LabelColor::Purple => write!(f, "purple"),
            LabelColor::Blue => write!(f, "blue"),
            LabelColor::Red => write!(f, "red"),
            LabelColor::Green => write!(f, "green"),
            LabelColor::Orange => write!(f, "orange"),
            LabelColor::Black => write!(f, "black"),
            LabelColor::Sky => write!(f, "yellow"),
            LabelColor::Pink => write!(f, "pink"),
            LabelColor::Lime => write!(f, "lime"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Label {
    pub id: String,
    pub name: String,
    pub color: String,
    #[serde(rename = "idBoard")]
    pub board_id: String,
}

impl Label {
    pub fn new(name: &str, color: LabelColor) -> Label {
        Label {
            id: String::new(),
            name: name.to_string(),
            color: color.to_string(),
            board_id: String::new(),
        }
    }
}

#[async_trait]
impl Creatable for Label {
    type Father = Board;

    async fn create(&self, father: &Self::Father) -> Option<Self> {
        Client::get().create_label(&father.id, &self).await
    }

    fn has_father(&self) -> bool {
        self.board_id != ""
    }

    async fn get_father(&self) -> Option<Self::Father> {
        if !self.has_father() {
            return None;
        }

        Client::get().get_board_by_id(&self.board_id).await
    }
}

#[async_trait]
impl Component for Label {
    type Child = Card;

    async fn get_all(&self) -> Option<Vec<Self::Child>> {
        if self.board_id.is_empty() {
            return None;
        }

        Client::get().get_cards_by_label(&self).await
    }

    fn name(&self) -> String {
        self.name.clone()
    }
    fn id(&self) -> String {
        self.id.clone()
    }
}
