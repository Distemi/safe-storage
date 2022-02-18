use actix_web::{HttpResponse, web};

use crate::DataBase;

#[derive(Deserialize, Debug)]
pub struct LoginPost {
    login: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    successful: bool,
    message: String,
}

#[post("/logout")]
pub async fn login(form: web::Form<LoginPost>, db: web::Data<DataBase>) -> HttpResponse {
    let usr = db.find_user_by_login(&form.login);
    if usr.is_none() {
        return HttpResponse::NotFound()
            .json(LoginResponse {
                successful: false,
                message: String::from("Login not exists"),
            });
    }
    let user = usr.unwrap();
    if user.password != form.password {
        return HttpResponse::NotFound()
            .json(LoginResponse {
                successful: false,
                message: String::from("Invalid password"),
            });
    }
    HttpResponse::Ok()
        .body("OK")
}