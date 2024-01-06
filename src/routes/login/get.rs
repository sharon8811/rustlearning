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
}