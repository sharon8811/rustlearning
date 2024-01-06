// use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, web, HttpRequest};
use actix_web_flash_messages::IncomingFlashMessages;
use minijinja::context;
use std::fmt::Write;
use crate::jinja::JinjaAppState;

pub async fn login_form(
    req: HttpRequest,
    flash_messages: IncomingFlashMessages,
    jinja_state: web::Data<JinjaAppState>,
) -> HttpResponse {
    let mut error_html = String::new();
    for m in flash_messages.iter() {
        writeln!(error_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }
    jinja_state.render_template("login.html", &req, context! {error_html => error_html})
//     HttpResponse::Ok()
//         .content_type(ContentType::html())
//         .body(format!(
//             r#"<!DOCTYPE html>
// <html lang="en">
// <head>
//     <meta http-equiv="content-type" content="text/html; charset=utf-8">
//     <title>Login</title>
// </head>
// <body>
//     {error_html}
//     <form action="/login" method="post">
//         <label>Username
//             <input
//                 type="text"
//                 placeholder="Enter Username"
//                 name="username"
//             >
//         </label>
//         <label>Password
//             <input
//                 type="password"
//                 placeholder="Enter Password"
//                 name="password"
//             >
//         </label>
//         <button type="submit">Login</button>
//     </form>
// </body>
// </html>"#,
//         ))
}