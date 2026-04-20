use crate::helpers::TestApp;

mod hm {
    use super::*;

    #[tokio::test]
    async fn test_alive() {
        let app = TestApp::new_hashmap().await.unwrap();
        let response = app.get_alive().await;
        assert_eq!(response.status().as_u16(), 200);
    } 
}

mod postgres {
    use super::*;

    #[tokio::test]
    async fn test_alive() {
        let app = TestApp::new_postgres().await.unwrap();
        let response = app.get_alive().await;
        assert_eq!(response.status().as_u16(), 200);
        app.cleanup().await;
    }
}
