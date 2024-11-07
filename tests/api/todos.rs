use todo_app::routes::todos::Response;

use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_200_with_empty_table() {
    let app = TestApp::new().await.unwrap();
    let response = app.get_todos().await;
    assert_eq!(response.status().as_u16(), 200);

    let response = response
        .json::<Response>()
        .await
        .expect("Could not deserialize response body to Response");

    assert_eq!(response.message, "Listing todos");
    assert!(response.content.is_none());

    app.cleanup().await;
}

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await.unwrap();

    let create_todo = serde_json::json!({
        "body": "foo",
    });

    let response = app.post_create_todo(&create_todo).await;
    assert_eq!(response.status().as_u16(), 201);

    let response = response
        .json::<Response>()
        .await
        .expect("Could not deserialize response body to Response");
    assert_eq!(response.message, "Todo created successfully");

    // -------

    let response = app.get_todos().await;
    assert_eq!(response.status().as_u16(), 200);

    let response = response
        .json::<Response>()
        .await
        .expect("Could not deserialize response body to Response");

    assert_eq!(response.message, "Listing todos");
    assert!(response.content.is_some());
    let todos = response.content.unwrap();
    assert_eq!(todos[0].id, 1);
    assert_eq!(todos[0].completed, false);
    assert_eq!(todos[0].body, "foo".to_string());

    app.cleanup().await;
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await.unwrap();

    let create_todo = serde_json::json!({
        "body": "foo",
    });

    let response = app.post_create_todo(&create_todo).await;
    assert_eq!(response.status().as_u16(), 201);

    let response = response
        .json::<Response>()
        .await
        .expect("Could not deserialize response body to Response");
    assert_eq!(response.message, "Todo created successfully");

    let test_cases = [
        serde_json::json!({
            "invalid": "foo",
        }),
        serde_json::json!({
            "": "bar",
        }),
        serde_json::json!({
            "response": "baz",
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_create_todo(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }

    app.cleanup().await;
}