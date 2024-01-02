use actix_web::http::header::ContentType;
use actix_web::HttpResponse;

pub async fn publish_newsletter_form() -> HttpResponse {
    let idempotency_key = uuid::Uuid::new_v4();
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("post_newsletter.html").replace("{idempotency_key}", &idempotency_key.to_string()))
}
