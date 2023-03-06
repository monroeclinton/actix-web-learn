use actix_web::{http, web, HttpRequest, HttpResponse, Responder};
use handlebars::Handlebars;
use serde_json::json;

pub async fn not_found(req: HttpRequest, handlebars: web::Data<Handlebars<'_>>) -> impl Responder {
    let data = json!({ "url": &req.uri().to_string() });
    let body = handlebars.render("not-found", &data).unwrap();

    HttpResponse::Ok()
        .status(http::StatusCode::NOT_FOUND)
        .body(body)
}
