use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Message {
    msg: String,
}

fn index(query: web::Query<Message>) -> impl Responder {
    format!("{}", query.msg)
}

#[rustfmt::skip]
fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind("127.0.0.1:8080")?
    .run()
}
