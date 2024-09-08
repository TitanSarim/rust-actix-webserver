use actix_web::{get, Responder};
use crate::routes::logging;

#[get("/home")]
pub async fn home() -> impl  Responder{
    logging("Get: /home");
    let response: &str = "Welcome to Actix web Server";
    response
}
