use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let address = app.address;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect(
            "Fai\
        led to execute request.",
        );

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

    let mut handles: Vec<tokio::task::JoinHandle<Result<(), std::io::Error>>> = vec![];
    for _ in 0..100 {
        let client = client.clone();
        let address = address.clone();
        handles.push(tokio::spawn(async move {
            let _ = client
                .get(format!("{}/health_check", &address))
                .send()
                .await;
            Ok(())
        }));
    }

    futures::future::join_all(handles).await;
    // env::remove_var("RUST_LOG");
}