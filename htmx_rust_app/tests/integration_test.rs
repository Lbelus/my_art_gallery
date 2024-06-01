#[cfg(test]
mod tests {
    use warp::http::StatusCode;
    use warp::test::request;

    #[tokio::test]
    async fn test_hello() {
        let res = request()
            .method("GET")
            .path("/hello")
            .reply(&super::hello())
            .await;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.body(), "Hello, HTMX + Rust!");
    }
}
