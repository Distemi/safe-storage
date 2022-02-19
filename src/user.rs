use actix_identity::Identity;
use actix_web::web;

use crate::database::DataBase;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    //#[serde(default = "NewUser")]
    pub login: String,
    //#[serde(default = "")]
    pub password: String,
    //#[serde(default = "/")]
    pub path: String,
    //#[serde(default = 128)]
    pub limit: u64,
    //#[serde(default = false)]
    pub admin: bool,
}


pub fn get_user(identify: &Identity, db: &web::Data<DataBase>) -> Option<User> {
    let login = identify.identity();
    if login.is_none() {
        return None;
    }
    let login = login.unwrap();
    let user = db.find_user_by_login(&login);
    if user.is_err() {
        return None;
    }
    return user.unwrap();
}