use actix_web::{get, web, App, HttpServer, Responder};

// #[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello, {}!",name)
}
async fn router() -> impl Responder {
    println!("page called");
    format!("I'm page!!!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("running on host!!");
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/hello/{name}").to(greet))
            .service(web::resource("/router").to(router))
    })
    .bind(("127.0.0.1", 8020))?
    .run()
    .await
}
//http://127.0.0.1:8020/hello/asdf