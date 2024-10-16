use crate::helpers::TestApp;

#[tokio::test]
async fn test_alive() {
    let app = TestApp::new().await.unwrap();
    let response = app.get_alive().await;
    assert_eq!(response.status().as_u16(), 200);
}