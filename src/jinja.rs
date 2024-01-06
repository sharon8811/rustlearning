use std::cell::RefCell;

use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse};
use minijinja::value::{Rest, Value};
use minijinja::{path_loader, Environment, Error, ErrorKind};

thread_local! {
    static CURRENT_REQUEST: RefCell<Option<HttpRequest>> = RefCell::default()
}


/// Binds the given request to a thread local for `url_for`.
fn with_bound_req<F, R>(req: &HttpRequest, f: F) -> R
where
    F: FnOnce() -> R,
{
    CURRENT_REQUEST.with(|current_req| *current_req.borrow_mut() = Some(req.clone()));
    let rv = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
    CURRENT_REQUEST.with(|current_req| current_req.borrow_mut().take());
    match rv {
        Ok(rv) => rv,
        Err(panic) => std::panic::resume_unwind(panic),
    }
}

pub struct JinjaAppState {
    env: minijinja::Environment<'static>,
}

impl JinjaAppState {

    pub fn create() -> JinjaAppState {
        let mut env = Environment::new();
        env.set_loader(path_loader("templates"));
        env.add_function("url_for", url_for);
        JinjaAppState { env }
    }

    pub fn render_template(&self, name: &str, req: &HttpRequest, ctx: Value) -> HttpResponse {
        with_bound_req(req, || {
            let tmpl = self.env.get_template(name).unwrap();
            let rv = tmpl.render(ctx).unwrap();
            HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(rv)
        })
    }
    
}

fn url_for(name: &str, args: Rest<String>) -> Result<Value, Error> {
    CURRENT_REQUEST.with(|current_req| {
        Ok(current_req
            .borrow()
            .as_ref()
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::InvalidOperation,
                    "url_for requires an http request",
                )
            })?
            .url_for(name, &args[..])
            .map_err(|err| {
                Error::new(ErrorKind::InvalidOperation, "failed to generate url").with_source(err)
            })?
            .to_string()
            .into())
    })
}