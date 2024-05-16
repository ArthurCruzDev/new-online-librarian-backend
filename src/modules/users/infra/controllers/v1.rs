use actix_web::{post, web, HttpResponse, Scope};

#[post("/")]
async fn create_user() -> HttpResponse {
    HttpResponse::Ok().body("create user")
}

pub fn get_user_scope() -> Scope {
    web::scope("/v1/users").service(create_user)
}