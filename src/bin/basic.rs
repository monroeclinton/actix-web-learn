use actix_web::{get, http, web, App, HttpResponse, HttpServer, Responder};
use actix_web_learn::not_found;
use actix_web_learn::operation;
use handlebars::Handlebars;

#[get("/")]
async fn home() -> impl Responder {
    "Hello World"
}

#[get("/greet/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

async fn manual_hello() -> impl Responder {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_templates_directory(".html", "./templates/")
        .unwrap();

    let data = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(home)
            .service(greet)
            .route("/hey", web::get().to(manual_hello))
            .service(
                web::scope("/operation")
                    .service(operation::multiply)
                    .service(operation::add),
            )
            .default_service(web::route().to(not_found::not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
