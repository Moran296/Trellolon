use lazy_static::lazy_static;
use std::env;

use crate::components::*;

pub const REQ_PREFIX: &str = "https://api.trello.com/1";
pub const _MEMBERS: &str = "members/me";

#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest::Client,
    auth: [(&'static str, &'static str); 2],
}

lazy_static! {
    static ref CLIENT: Client = Client::new();
    static ref KEY: String = env::var("TRELLO_KEY").unwrap();
    static ref TOKEN: String = env::var("TRELLO_TOKEN").unwrap();
}

//API
impl Client {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        let auth = [
            ("key", KEY.as_ref()),
            ("token", TOKEN.as_ref()),
        ];
        Client { client, auth }
    }

    pub fn get() -> &'static Self {
        &CLIENT
    }

    pub async fn get_boards(&self) -> Option<Vec<Board>> {
        let url = format!("{REQ_PREFIX}/{_MEMBERS}/boards?");
        let resp = self.client.get(&url).form(&self.auth).send().await.ok()?;
        let boards = resp.json().await.ok()?;
        Some(boards)
    }

    pub async fn get_board(&self, name: &str) -> Option<Board> {
        let boards = self.get_boards().await?;
        boards.get_by_name(name)
    }

    pub async fn get_board_by_id(&self, id: &str) -> Option<Board> {
        let boards = self.get_boards().await?;
        boards.get_by_id(id)
    }

    pub async fn get_list_by_id(&self, list_id: &str) -> Option<List> {
        let url = format!("{REQ_PREFIX}/lists/{list_id}", list_id = list_id);
        let resp = self.client.get(&url).form(&self.auth).send().await.ok()?;
        let list = resp.json().await.ok()?;
        Some(list)
    }

    pub async fn get_lists(&self, board_id: &str) -> Option<Vec<List>> {
        let url = format!("{REQ_PREFIX}/boards/{board_id}/lists/?",);
        let resp = self.client.get(&url).form(&self.auth).send().await.ok()?;
        let lists = resp.json().await.ok()?;
        Some(lists)
    }

    pub async fn get_cards(&self, list_id: &str) -> Option<Vec<Card>> {
        let url = format!("{REQ_PREFIX}/lists/{list_id}/cards/?");
        let resp = self.client.get(&url).form(&self.auth).send().await.ok()?;
        let cards = resp.json().await.ok()?;
        Some(cards)
    }

    pub async fn get_board_cards(&self, board_id: &str) -> Option<Vec<Card>> {
        let lists = self.get_lists(board_id).await?;
        let mut cards = Vec::new();
        for list in lists {
            cards.append(&mut self.get_cards(&list.id).await?);
        }

        if cards.is_empty() {
            None
        } else {
            Some(cards)
        }
    }

    pub async fn get_cards_by_label(&self, label: &Label) -> Option<Vec<Card>> {
        let cards = self.get_board_cards(&label.board_id).await?;

        let filtered: Vec<Card> = cards
            .iter()
            .filter(|card| card.labels.iter().any(|label_id| label_id == &label.id))
            .cloned()
            .collect();

        if filtered.is_empty() {
            None
        } else {
            Some(filtered)
        }
    }

    pub async fn get_lables(&self, board_id: &str) -> Option<Vec<Label>> {
        let url = format!("{REQ_PREFIX}/boards/{board_id}/labels/?",);
        let resp = self.client.get(&url).form(&self.auth).send().await.ok()?;
        let labels = resp.json().await.ok()?;
        Some(labels)
    }

    pub async fn get_label_by_id(&self, label_id: &str) -> Option<Label> {
        let url = format!("{REQ_PREFIX}/labels/{label_id}/?");
        let resp = self.client.get(&url).form(&self.auth).send().await.ok()?;
        let label = resp.json().await.ok()?;
        Some(label)
    }

    pub async fn get_card_labels(&self, card: &Card) -> Option<Vec<Label>> {
        if !card.has_labels() {
            return None;
        }

        let mut labels: Vec<Label> = Vec::new();
        for label_id in &card.labels {
            let label = self.get_label_by_id(&label_id).await;
            if let Some(label) = label {
                labels.push(label);
            }
        }

        if labels.is_empty() {
            return None;
        }

        Some(labels)
    }

    pub async fn create_list(&self, board_id: &str, name: &str) -> Option<List> {
        let url = format!("{REQ_PREFIX}/lists?");
        let resp = self
            .client
            .post(&url)
            .form(&[
                ("name", name),
                ("idBoard", board_id),
                self.auth[0],
                self.auth[1],
            ])
            .send()
            .await
            .ok()?;
        let list = resp.json().await.ok()?;
        Some(list)
    }

    pub async fn create_card(&self, list_id: &str, card: &Card) -> Option<Card> {
        let url = format!("{REQ_PREFIX}/lists/{list_id}/cards/?");

        //create form
        let mut form: Vec<(&str, &str)> = vec![
            ("name", &card.name),
            ("desc", &card.description),
            ("idList", list_id),
        ];

        form.push(self.auth[0]);
        form.push(self.auth[1]);

        let resp = self.client.post(&url).form(&form).send().await.ok()?;
        let card = resp.json().await.ok()?;
        Some(card)
    }

    pub async fn create_label(&self, board_id: &str, label: &Label) -> Option<Label> {
        let url = format!("{REQ_PREFIX}/labels?");
        let form: Vec<(&str, &str)> = vec![
            ("name", &label.name),
            ("color", &label.color),
            ("idBoard", board_id),
            self.auth[0],
            self.auth[1],
        ];

        let resp = self.client.post(&url).form(&form).send().await.ok()?;
        let label = resp.json().await.ok()?;
        Some(label)
    }

    pub async fn add_label_to_card(&self, card: Card, label_id: &str) -> Option<Card> {
        let url = format!("{REQ_PREFIX}/cards/{}/idLabels?", card.id);
        println!("{}", url);

        let form: Vec<(&str, &str)> = vec![("value", label_id), self.auth[0], self.auth[1]];

        let resp = self.client.post(&url).form(&form).send().await.ok()?;
        if resp.status() == reqwest::StatusCode::OK {
            let card = self.get_cards(&card.id_list).await?.get_by_id(&card.id)?;
            Some(card)
        } else {
            None
        }
    }

    pub async fn add_comment_to_card(&self, card: Card, comment: &str) -> Option<Card> {
        let url = format!("{REQ_PREFIX}/cards/{}/actions/comments?", &card.id);
        let form: Vec<(&str, &str)> = vec![
            ("text", comment),
            self.auth[0],
            self.auth[1],
        ];

        let resp = self.client.post(&url).form(&form).send().await.ok()?;
        if resp.status() == reqwest::StatusCode::OK {
            let card = self.get_cards(&card.id_list).await?.get_by_id(&card.id)?;
            Some(card)
        } else {
            None
        }
    }

    pub async fn get_comments_from_card(&self, card: &Card) -> Option<Vec<card::Comment>> {
        let url = format!("{REQ_PREFIX}/cards/{}/actions?filter=commentCard", &card.id);
        let resp = self.client.get(&url).form(&self.auth).send().await.ok()?;
        let comments = resp.json().await.ok()?;
        Some(comments)
    }

}
