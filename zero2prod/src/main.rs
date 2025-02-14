use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::HttpRequest;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn health_check() -> impl Responder { 
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/health_check", web::get().to(health_check))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

