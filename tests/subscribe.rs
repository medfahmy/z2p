use tempo::spawn;

#[tokio::test]
async fn sub_valid_form() {
    let (addr, pool) = spawn().await;
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let res = client
        .post(format!("http://{}/subscribe", addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();
    assert_eq!(201, res.status().as_u16());

    let saved = sqlx::query!("select email, name from subscriptions")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");

    let res = client
        .post(format!("http://{}/subscribe", addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();

    assert_eq!(409, res.status().as_u16());
}

#[tokio::test]
async fn sub_invalid_form() {
    let (addr, _) = spawn().await;
    let client = reqwest::Client::new();

    let cases = [
        ("name=le%20guin", "missing email"),
        ("email=ursula_le_guin%40gmail.com", "missing name"),
        ("", "missing name and email"),
    ];

    let res = client
        .post(format!("http://{}/subscribe", addr))
        // .header("Content-Type", "application/x-www-form-urlencoded")
        // .body(body)
        .send()
        .await
        .unwrap();

    assert_eq!(415, res.status().as_u16(), "no headers and body");

    let text = res.text().await.unwrap();
    tracing::info!("{}", text);

    for (body, msg) in cases {
        let res = client
            .post(format!("http://{}/subscribe", addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .unwrap();
        assert_eq!(422, res.status().as_u16(), "{}: {}", body, msg);

        let text = res.text().await.unwrap();
        tracing::info!("{}", text);
    }
}
