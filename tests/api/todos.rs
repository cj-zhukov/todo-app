use todo_app::routes::todos::Response;

use crate::helpers::TestApp;

// #[tokio::test]
// async fn should_return_422_if_malformed_input() {
//     let app = TestApp::new().await.unwrap();

//     let test_cases = [
//         serde_json::json!({
//             "val": "foo",
//         }),
//         serde_json::json!({
//             "val": "bar",
//         }),
//         serde_json::json!({
//             "val": "baz",
//         }),
//     ];

//     for test_case in test_cases.iter() {
//         let response = app.post_create_todo(test_case).await;
//         assert_eq!(
//             response.status().as_u16(),
//             422,
//             "Failed for input: {:?}",
//             test_case
//         );
//     }
// }

// #[tokio::test]
// async fn should_return_201_if_valid_input() {
//     let app = TestApp::new().await.unwrap();

//     let test_case = serde_json::json!({
//         "body": "my todo task",
//     });

//     let response = app.post_create_todo(&test_case).await;
//     assert_eq!(response.status().as_u16(), 201);
// }

// #[tokio::test]
// async fn should_return_409_if_email_already_exists() {
//     let app = TestApp::new().await.unwrap();

//     let input = serde_json::json!({
//         "body": "foo",
//     });

//     let _ = app.post_create_todo(&input).await;
//     let response = app.post_create_todo(&input).await; 
//     assert_eq!(response.status().as_u16(), 409);

//     assert_eq!(
//         response
//             .json::<Response>()
//             .await
//             .expect("Could not deserialize response body to ErrorResponse")
//             .error,
//         "User already exists".to_owned()
//     );
// }