use crate::{client::Client, Component, Creatable, Label, List, Board};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    pub id: String,
    pub name: String,
    #[serde(rename = "idList")]
    pub id_list: String,
    #[serde(rename = "idBoard")]
    pub id_board: String,
    #[serde(rename = "desc")]
    pub description: String,
    #[serde(rename = "idLabels")]
    pub labels: Vec<String>,
}

impl Card {
    pub fn new(name: &str, description: String) -> Card {
        Card {
            id: String::new(),
            name: name.to_string(),
            id_list: "".to_string(),
            id_board: "".to_string(),
            description: description,
            labels: Vec::new(),
        }
    }

    pub async fn get(id: &str) -> Option<Self> {
        Client::get().get_card_by_id(id).await
    }

    pub fn has_labels(&self) -> bool {
        !self.labels.is_empty()
    }

    pub async fn add_label(self, label: &Label) -> Option<Card> {
        Client::get().add_label_to_card(self, &label.id).await
    }

    pub async fn get_board(&self) -> Option<Board> {
        Client::get().get_board_by_id(&self.id_board).await
    }

    pub async fn add_comment(self, comment: &str) -> Option<Card> {
        Client::get().add_comment_to_card(self, comment).await
    }

    pub async fn get_comments(&self) -> Option<Vec<String>> {
        let comments = Client::get().get_comments_from_card(&self).await;
        if comments.is_none() {
            return None;
        }

        let texts: Vec<String> = comments
            .unwrap()
            .iter()
            .map(|comment| comment.data.text.clone())
            .collect();

        if texts.is_empty() {
            return None;
        }

        Some(texts)
    }
}

#[async_trait]
impl Creatable for Card {
    type Father = List;

    async fn create(&self, father: &Self::Father) -> Option<Self> {
        Client::get().create_card(&father.id, &self).await
    }

    fn has_father(&self) -> bool {
        self.id_list != ""
    }

    async fn get_father(&self) -> Option<Self::Father> {
        if !self.has_father() {
            return None;
        }

        Client::get().get_list_by_id(&self.id_list).await
    }
}

#[async_trait]
impl Component for Card {
    type Child = Label;

    async fn get_all(&self) -> Option<Vec<Self::Child>> {
        Client::get().get_card_labels(&self).await
    }

    fn name(&self) -> String {
        self.name.clone()
    }
    fn id(&self) -> String {
        self.id.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comment {
    data: Data,
}
