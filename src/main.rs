mod routes;
use actix_web::{HttpServer, App};
use routes::*;


#[actix_web::main]
async fn main() -> std::io::Result<()>{

    let server = HttpServer::new(|| App::new().service(home).service(hello_user).service((create_new_user))
    ).bind(("127.0.0.1", 8000))?
        .run();

    println!("Server is running at localhost:8000");
    server.await
}
