use actix_web::{get, post, web, App, HttpServer, Responder};
use std::collections::HashSet;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;

#[get("/count")]
async fn count(products: web::Data<Mutex<HashSet<u32>>>) -> impl Responder {
    format!("{}", products.lock().unwrap().len())
}

#[get("/get_products")]
async fn get_products(products: web::Data<Mutex<HashSet<u32>>>) -> impl Responder {
    products
        .lock()
        .unwrap()
        .iter()
        .map(|s| format!("{} ,", s))
        .collect::<String>()
}

#[post("/add_product")]
async fn add_product(
    products: web::Data<Mutex<HashSet<u32>>>,
    counter: web::Data<AtomicU32>,
) -> impl Responder {
    let a = counter.fetch_add(1, Ordering::SeqCst);
    products.lock().unwrap().insert(a);
    format!("{}", a)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let products = web::Data::new(Mutex::new(HashSet::<u32>::new()));
    let counter = web::Data::new(AtomicU32::new(0u32));

    HttpServer::new(move || {
        App::new()
            .app_data(products.clone())
            .app_data(counter.clone())
            .service(count)
            .service(add_product)
            .service(get_products)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
