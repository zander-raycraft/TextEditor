use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::http::header;
use actix_web::middleware::DefaultHeaders;

async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello, World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                DefaultHeaders::new()
                    .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "http://localhost:3000")
            )
            .service(web::resource("/api/hello").to(hello))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
