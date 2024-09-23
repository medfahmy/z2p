#[tokio::test]
async fn health_check() {
    let (addr, _) = tempo::spawn().await;
    let client = reqwest::Client::new();
    let res = client
        .get(format!("http://{}", addr))
        .header("Host", "localhost")
        .send()
        .await
        .unwrap();

    assert!(res.status().is_success());
    assert_eq!(res.content_length(), Some(0));
}

// #[tokio::test]
// async fn json() {
//     let addr = run().await;
//
//     let client = reqwest::Client::new();

// let res = client
//     .post(format!("http://{}/json", addr))
//     .header("Host", "localhost")
//     .body(serde_json::to_vec(&json!([1, 2, 3, 4])).unwrap())
//     .send()
//     .await
//     .unwrap();
//
// assert!(res.status().is_success());
// // assert_eq!(res.content_length(), Some(0));
// // let body = res.into_body().collect().await.unwrap().to_bytes();
// //
// let body = res.json().await.unwrap();
// let body: Value = serde_json::from_slice(&body).unwrap();
// assert_eq!(body, json!({ "data": [1, 2, 3, 4] }));

// let response = router
//     .oneshot(
//         Request::builder()
//             .method(http::Method::POST)
//             .uri("/json")
//             .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
//             .body(Body::from(
//                 serde_json::to_vec(&json!([1, 2, 3, 4])).unwrap(),
//             ))
//             .unwrap(),
//     )
//     .await
//     .unwrap();
//
// assert_eq!(response.status(), StatusCode::OK);
//
// let body = response.into_body().collect().await.unwrap().to_bytes();
// let body: Value = serde_json::from_slice(&body).unwrap();
// assert_eq!(body, json!({ "data": [1, 2, 3, 4] }));
// }

//
// #[tokio::test]
// async fn hello_world() {
//     let router = router();
//
//     let response = router
//         .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
//         .await
//         .unwrap();
//
//     assert_eq!(response.status(), StatusCode::OK);
//
//     let body = response.into_body().collect().await.unwrap().to_bytes();
//     assert!(body.is_empty());
// }
//
// #[tokio::test]
// async fn not_found() {
//     let router = router();
//
//     let response = router
//         .oneshot(
//             Request::builder()
//                 .uri("/does-not-exist")
//                 .body(Body::empty())
//                 .unwrap(),
//         )
//         .await
//         .unwrap();
//
//     assert_eq!(response.status(), StatusCode::NOT_FOUND);
//     let body = response.into_body().collect().await.unwrap().to_bytes();
//     assert!(body.is_empty());
// }
//
//
// // You can use `ready()` and `call()` to avoid using `clone()`
// // in multiple request
// #[tokio::test]
// async fn multiple_request() {
//     let mut router = router().into_service();
//
//     let request = Request::builder().uri("/").body(Body::empty()).unwrap();
//     let response = ServiceExt::<Request<Body>>::ready(&mut router)
//         .await
//         .unwrap()
//         .call(request)
//         .await
//         .unwrap();
//     assert_eq!(response.status(), StatusCode::OK);
//
//     let request = Request::builder().uri("/").body(Body::empty()).unwrap();
//     let response = ServiceExt::<Request<Body>>::ready(&mut router)
//         .await
//         .unwrap()
//         .call(request)
//         .await
//         .unwrap();
//     assert_eq!(response.status(), StatusCode::OK);
// }
//
// // Here we're calling `/requires-connect-info` which requires `ConnectInfo`
// //
// // That is normally set with `Router::into_make_service_with_connect_info` but we can't easily
// // use that during tests. The solution is instead to set the `MockConnectInfo` layer during
// // tests.
// #[tokio::test]
// async fn with_into_make_service_with_connect_info() {
//     let mut router = router()
//         .layer(MockConnectInfo(SocketAddr::from(([0, 0, 0, 0], 3000))))
//         .into_service();
//
//     let request = Request::builder()
//         .uri("/requires-connect-info")
//         .body(Body::empty())
//         .unwrap();
//     let response = router.ready().await.unwrap().call(request).await.unwrap();
//     assert_eq!(response.status(), StatusCode::OK);
// }
