use crate::client::Client;
use crate::components::Card;
use crate::components::*;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct List {
    pub id: String,
    pub name: String,
    #[serde(rename = "idBoard")]
    pub board_id: String,
    pub closed: bool,
}

impl List {
    pub async fn get(id: &str) -> Option<Self> {
        Client::get().get_list_by_id(id).await
    }
}

#[async_trait]
impl Component for List {
    type Child = Card;

    async fn get_all(&self) -> Option<Vec<Self::Child>> {
        Client::get().get_cards(&self.id).await
    }

    fn name(&self) -> String {
        self.name.clone()
    }
    fn id(&self) -> String {
        self.id.clone()
    }
}

#[async_trait]
impl Creatable for List {
    type Father = Board;

    async fn create(&self, father: &Self::Father) -> Option<Self> {
        Client::get().create_list(&father.id, &self.name).await
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
