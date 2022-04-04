mod client;
pub mod components;
pub use components::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn i1() {
        let c = client::Client::get();
        let b = c.get_board("TestBoard").await.unwrap();
        println!("{:#?}", b);

        let l = b.get_by_name("List1").await.unwrap();
        println!("{:#?}", l);

        let c: Card = l.get_by_name("Card1").await.unwrap();

        println!("{:#?}", c);

        assert!(true);
    }
}
