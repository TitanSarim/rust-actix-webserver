use actix_web:: {http::StatusCode, post, web::Json, Responder};
use serde::Serialize;
use crate::routes::{logging, User};

#[derive(Serialize)]
struct CreateUserResponder {
    id: u32,
    user: User
}
#[post("/user/create")]
pub async  fn create_new_user(user: Json<User>) -> impl Responder{
    logging("POST: /user/create");

    (Json(CreateUserResponder {
        id: 1,
        user: user.0,
    }), StatusCode:: CREATED)
}