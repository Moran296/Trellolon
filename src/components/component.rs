use async_trait::async_trait;

#[allow(dead_code)]
pub const REQ_PREFIX: &str = "https://api.trello.com/1/members/me/";
#[allow(dead_code)]
pub const KEY_AND_TOKEN_FORMAT: &str = "key={}&token={}";

#[async_trait]
pub trait Creatable: Sized {
    type Father;

    async fn create(&self, father: &Self::Father) -> Option<Self>;
}

pub trait GetByNameAndId: Sized {
    type Output;

    fn get_by_name(&self, name: &str) -> Option<Self::Output>;
    fn get_by_id(&self, id: &str) -> Option<Self::Output>;
}

impl<T> GetByNameAndId for Vec<T>
where
    T: Sized + Clone + Component,
{
    type Output = T;

    fn get_by_name(&self, name: &str) -> Option<T> {
        self.iter().find(|&b| b.name() == name).cloned()
    }

    fn get_by_id(&self, id: &str) -> Option<T> {
        self.iter().find(|&b| b.id() == id).cloned()
    }
}

#[async_trait]
pub trait Component {
    type Child: Sized + Component + Clone;

    async fn get_all(&self) -> Option<Vec<Self::Child>>;

    async fn get_by_name(&self, name: &str) -> Option<Self::Child> {
        self.get_all().await?.get_by_name(name)
    }

    async fn get_by_id(&self, id: &str) -> Option<Self::Child> {
        self.get_all().await?.get_by_id(id)
    }

    fn name(&self) -> String;
    fn id(&self) -> String;
}
