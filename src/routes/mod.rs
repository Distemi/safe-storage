pub mod auth;
pub mod user_info;

#[derive(Serialize, Deserialize, Debug)]
pub struct MessResponse<T> {
    successful: bool,
    message: T,
}
