use super::component::Component;
use super::component::Creatable;
use crate::client::Client;
use crate::components::Label;
use crate::components::List;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    pub id: String,
    pub name: String,
    #[serde(rename = "idList")]
    pub id_list: String,
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
            description: description,
            labels: Vec::new(),
        }
    }

    pub fn has_labels(&self) -> bool {
        !self.labels.is_empty()
    }

    pub async fn add_label(self, label: &Label) -> Option<Card> {
        Client::get().add_label_to_card(self, &label.id).await
    }
}

#[async_trait]
impl Creatable for Card {
    type Father = List;

    async fn create(&self, father: &Self::Father) -> Option<Self> {
        Client::get().create_card(&father.id, &self).await
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
