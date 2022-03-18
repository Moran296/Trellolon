mod client;
mod components;
use components::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn i1() {
        let c = client::Client::new();
        let b = c.get_board("TestBoard").await.unwrap();
        println!("{:#?}", b);

        let l = b.get_by_name("List1").await.unwrap();
        println!("{:#?}", l);

        let c = l.get_all().await.unwrap();
        println!("{:#?}", c);

        let label = Label::new("laboo", label::LabelColor::Lime)
            .create(&b)
            .await
            .unwrap();
        println!("created label: {:#?}", label);

        for card in c {
            println!("card: {:#?}", card);

            if card.has_labels() {
                let labels = card.get_all().await.unwrap();
                println!("label {:#?}", labels);
            } else {
                let new_card = card.add_label(&label).await.unwrap();
                println!("new card: {:#?}", new_card);
            }
        }

        assert!(true);
    }
}
