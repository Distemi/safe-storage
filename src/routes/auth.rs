use actix_identity::Identity;
use actix_web::{HttpResponse, web};

use crate::database::DataBase;
use crate::routes::MessResponse;

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginPost {
    login: String,
    password: String,
}

#[post("/api/login")]
pub async fn login(form: web::Json<LoginPost>, db: web::Data<DataBase>, identify: Identity) -> HttpResponse {
    let usr = db.find_user_by_login(&form.login);
    if usr.is_err() {
        return HttpResponse::InternalServerError()
            .json(MessResponse {
                successful: false,
                message: String::from("Failed to get users"),
            });
    }
    let usr = usr.unwrap();
    if usr.is_none() {
        return HttpResponse::NotFound()
            .json(MessResponse {
                successful: false,
                message: String::from("Login not exists"),
            });
    }
    let user = usr.unwrap();
    if !user.password.eq(&form.password) {
        return HttpResponse::NotFound()
            .json(MessResponse {
                successful: false,
                message: String::from("Invalid password"),
            });
    }
    identify.remember(user.login.clone());
    HttpResponse::Ok()
        .json(MessResponse {
            successful: true,
            message: String::from("Successful login!"),
        })
}