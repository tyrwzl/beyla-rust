use std::io;

use actix_web::{web, App, HttpServer};

async fn hello() -> &'static str {
    "Hello world! from beyla-rust"
}


#[actix_web::main]
async fn main() -> io::Result<()> {

    HttpServer::new(move || {
        App::new()
            .service(web::resource("/hello").to(hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
