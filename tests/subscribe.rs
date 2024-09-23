#[tokio::test]
async fn subscribe_valid_form() {
    let addr = newsletter::spawn().await;
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let res = client
        .post(format!("http://{}/subscribe", addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();

    assert_eq!(200, res.status().as_u16());
}

#[tokio::test]
async fn subscribe_invalid_form() {
    let addr = newsletter::spawn().await;
    let client = reqwest::Client::new();

    let cases = [
        ("name=le%20guin", "missing email"),
        ("email=ursula_le_guin%40gmail.com", "missing name"),
        ("", "missing name and email"),
    ];

    for (body, msg) in cases {
        let res = client
            .post(format!("http://{}/subscribe", addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .unwrap();

        assert_eq!(200, res.status().as_u16());
    }
}
