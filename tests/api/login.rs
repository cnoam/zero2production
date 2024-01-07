//! tests/api/login.rs
use crate::helpers::assert_is_redirect_to;
use crate::helpers::spawn_app;
#[tokio::test]
async fn an_error_flash_message_is_set_on_failure() {
// Arrange
    let app = spawn_app().await;
// Act
    let login_body = serde_json::json!({
"username": "random-username",
"password": "random-password"
});
    let response = app.post_login(&login_body).await;
    assert_is_redirect_to(&response,"/login");
    //let flash_cookie = response.cookies().find(|c| c.name() == "_flash").unwrap();
    //assert_eq!(flash_cookie.value(), "Authentication failed");

    // Act - Part 2
    let html_page = app.get_login_html().await;
    assert!(html_page.contains("<p><i>Authentication failed</i></p>"));

    // Act - Part 3 - Reload the login page
    let html_page = app.get_login_html().await;
    assert!(!html_page.contains("Authentication failed"));
}