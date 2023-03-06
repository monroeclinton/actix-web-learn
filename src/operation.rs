use actix_web::{
    body::BoxBody, get, http::header, post, web, HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Nums {
    first: u64,
    second: u64,
}

#[get("/multiply")]
pub async fn multiply(nums: web::Query<Nums>) -> impl Responder {
    format!("Result: {}!", nums.first * nums.second)
}

#[derive(Debug, Serialize)]
struct Operation {
    operation: String,
    result: u64,
}

impl Responder for Operation {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(header::ContentType::json())
            .body(body)
    }
}

#[post("/add")]
pub async fn add(nums: web::Json<Nums>) -> impl Responder {
    Operation {
        operation: "add".to_string(),
        result: nums.first + nums.second,
    }
}
