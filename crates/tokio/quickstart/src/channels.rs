pub mod channels {
    use tokio::sync::{mpsc, oneshot};

    pub fn test() {
        println!("success");
    }

    pub async fn r#channel() {
        let (tx, mut rx) = mpsc::channel(32);
        let tx2 = tx.clone();

        tokio::spawn(async move {
            tx.send("send from first handle").await.unwrap();
        });

        tokio::spawn(async move {
            tx2.send("send from second handle").await.unwrap();
        });

        while let Some(msg) = rx.recv().await {
            println!("GOT = {}", msg);
        }
    }

    pub async fn r#oneshot() {
        let (resp_tx, resp_rx) = oneshot::channel();
        tokio::spawn(async move {
            // Send the GET request
            resp_tx.send("cmd".to_string()).unwrap();
        });
        // Await the response
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    }
}
