use crate::client::Client;
use crate::components::Component;
use crate::components::List;
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
