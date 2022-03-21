use crate::{client::Client, Component, List, Label};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Board {
    pub id: String,
    pub name: String,
    #[serde(rename = "desc")]
    pub description: String,
}

#[async_trait]
impl Component for Board {
    type Child = List;

    async fn get_all(&self) -> Option<Vec<Self::Child>> {
        Client::get().get_lists(&self.id).await
    }

    fn name(&self) -> String {
        self.name.clone()
    }
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl Board {
    pub async fn get(name: &str) -> Option<Self> {
        Client::get().get_board(name).await
    }

    pub async fn get_labels(&self) -> Option<Vec<Label>> {
        Client::get().get_labels(&self.id).await
    }

    pub async fn is_label_color_available(&self, color: &str) -> bool {
        let labels = self.get_labels().await;
        if labels.is_none() {
            return true;
        }

        labels.unwrap().iter().all(|label| label.color != color)
    }
}