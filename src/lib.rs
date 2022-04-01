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

        let c = l.get_by_name("Card1").await.unwrap();

        let list = match c.get_father().await {
            Some(list) => list,
            _ => panic!("No list found"),
        };
        let board = match list.get_father().await {
            Some(board) => board,
            _ => panic!("No board found"),
        };
        let labels = match board.get_labels().await {
            Some(labels) => labels,
            _ => panic!("No labels found"),
        };      

        println!("{:#?}", labels);

        assert!(true);
    }
}
