use actix_identity::Identity;
use actix_web::{HttpResponse, web};

use crate::database::DataBase;
use crate::routes::MessResponse;
use crate::user::get_user;

#[derive(Serialize)]
pub struct UserInfoResponse {
    successful: bool,
    login: String,
    path: String,
    limit: u64,
    admin: Option<bool>,
}

#[get("/api/user_info")]
pub async fn user_info(identify: Identity, db: web::Data<DataBase>) -> HttpResponse {
    let user = get_user(&identify, &db);
    if user.is_none() {
        return HttpResponse::InternalServerError()
            .json(MessResponse {
                successful: false,
                message: "Invalid user",
            });
    }
    let user = user.unwrap();
    return HttpResponse::Ok().json(UserInfoResponse {
        successful: true,
        login: user.login,
        path: user.path,
        limit: user.limit,
        admin: if user.admin {
            Some(true)
        } else {
            None
        },
    });
}