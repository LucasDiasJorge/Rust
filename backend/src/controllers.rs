// src/controllers.rs

use actix_web::{web, HttpResponse};
use crate::services::UserService;

pub async fn get_user_by_id(path: web::Path<u64>) -> HttpResponse {
    let user_id = path.into_inner();
    let user = UserService::get_user_by_id(user_id);

    HttpResponse::Ok().json(user)
}
