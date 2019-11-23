use actix_web::{web, App, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Query {
    message: String,
}

fn index(query: web::Query<Query>) -> impl Responder {
    format!("{}", query.message)
}

#[rustfmt::skip]
fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind("localhost:12345")?
    .run()
}
