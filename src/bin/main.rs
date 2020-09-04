use actix_web::{App, HttpServer};
use rust_demo::routes::build_routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(build_routes))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
