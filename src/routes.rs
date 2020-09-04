use crate::demolition::demolition;
use crate::models::demo::{Demo, Demos};
use crate::response_handler::{handle, Actions};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
/// trait syntax: https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits
/// by returning `impl Responder` we are showing that these functions return a type that implements the Responder `trait`
/// traits are a way to define shared functionality
/// traits allow genericizing response types (avoiding explicit typing) by allowing the return of any type that implements the returned trait
/// However, this specific style of returning a trait does require that a single type be returned from the function.

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, you have reached the demo api!")
}
/// get all demo
#[get("/demo-lition")]
async fn demo_lition() -> impl Responder {
    handle(Actions::ReadOne(demolition(Demos::read())))
}
/// get all demo
#[get("/demo")]
async fn read_all() -> impl Responder {
    handle(Actions::ReadMany(Demos::read()))
}
/// get demo by id
#[get("/demo/{id}")]
async fn read_one(id: web::Path<i32>) -> impl Responder {
    handle(Actions::ReadOne(Demos::read_by_id(id.into_inner())))
}
/// create demo by id
#[post("/demo")]
async fn create_demo(demo: web::Json<Demo>) -> impl Responder {
    handle(Actions::Create(Demos::create(demo.into_inner())))
}

/// update demo by id
#[put("/demo/{id}")]
async fn update_demo(id: web::Path<i32>, demo: web::Json<Demos>) -> impl Responder {
    handle(Actions::Update(Demos::update(
        id.into_inner(),
        demo.into_inner(),
    )))
}

/// update demo by id∂
#[delete("/demo/{id}")]
async fn delete_demo(id: web::Path<i32>) -> impl Responder {
    handle(Actions::Delete(Demos::delete(id.into_inner())))
}

/// gather routes for initialization with the actix server
/// pub allows us to use this function outside this module
pub fn build_routes(config: &mut web::ServiceConfig) {
    config.service(hello);
    config.service(read_all);
    config.service(read_one);
    config.service(create_demo);
    config.service(update_demo);
    config.service(delete_demo);
    config.service(demo_lition);
}
