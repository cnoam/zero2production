//!src/routes/login/get.rs
use actix_web::HttpResponse;

use actix_web::http::header::ContentType;
//use actix_web::cookie::{Cookie, time::Duration};
use actix_web_flash_messages::{IncomingFlashMessages,Level};
use std::fmt::Write;

pub async fn login_form(flash_messages : IncomingFlashMessages)
 -> HttpResponse {
    // let error_html = match request.cookie("_flash") {
    //     None => "".into(),
    //     Some(cookie) => {
    //         format!("<p><i>{}</i></p>", cookie.value())
    //     }
    // };

    let mut error_html = String::new();
    for m in flash_messages.iter() {
        writeln!(error_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }
    HttpResponse::Ok()
        .content_type(ContentType::html())
        // remove the cookie by duration = 0
        // or use Reponse::add_removal_cookie()
        // .cookie(
        //     Cookie:: build("_flash", "")
        //         .max_age(Duration::ZERO)
        //         .finish(),
        // )
        .body( format!(
                r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Login</title>
</head>
<body>
    {error_html}
    <form action="/login" method="post">
        <label>Username
            <input
                type="text"
                placeholder="Enter Username"
                name="username"
            >
        </label>
        <label>Password
            <input
                type="password"
                placeholder="Enter Password"
                name="password"
            >
        </label>
        <button type="submit">Login</button>
    </form>
</body>
</html>"#,
            ))
}

