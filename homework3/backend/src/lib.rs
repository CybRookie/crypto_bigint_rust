
use actix_files as fs;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok()
        .body("data")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body("Hello world TEST!")
}

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .service(fs::Files::new("/", "../frontend/dist").index_file("index.html"))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
