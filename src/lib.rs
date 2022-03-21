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

        let c = l.get_by_name("Moran the best").await.unwrap();
        println!("{:#?}", c);

        let comments = c.get_comments().await.unwrap();
        println!("comments {:#?}", comments);

        let c = c.add_comment("Hello, world!").await.unwrap();

        let comments = c.get_comments().await.unwrap();
        println!("comments {:#?}", comments);

        assert!(true);
    }
}
