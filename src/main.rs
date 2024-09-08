mod routes;
mod database;
use database::*;

use actix_web::{HttpServer, App};
use sqlx::{MySqlPool};
use routes::*;


#[actix_web::main]
async fn main() -> std::io::Result<()>{

    let database: MySqlPool = database_connection().await.expect("Database connection Failed");

    println!("Database connection Success");

    let server = HttpServer::new(move || App::new().app_data(database.clone()).service(home).service(hello_user).service(create_new_user)
    ).bind(("127.0.0.1", 8000))?
        .run();

    println!("Server is running at localhost:8000");
    server.await
}
