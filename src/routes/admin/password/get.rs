use actix_web::{HttpResponse, HttpRequest, web};
use actix_web_flash_messages::IncomingFlashMessages;
use minijinja::context;
use std::fmt::Write;

use crate::jinja::JinjaAppState;

pub async fn change_password_form(
    req: HttpRequest,
    flush_messages: IncomingFlashMessages,
    jinja_state: web::Data<JinjaAppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut msg_html = String::new();
    for m in flush_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    Ok(jinja_state.render_template("reset_password.html", &req, context! {msg_html => msg_html}))
}