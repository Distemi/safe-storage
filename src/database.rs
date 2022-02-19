use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Mutex;

use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

use crate::SResult;
use crate::user::User;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub cookie_key: String,
    pub port: u16,
    pub host: String,
    pub users: Vec<User>,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let mut rng = thread_rng();
        let default = Config {
            cookie_key: (0..rng.gen_range(50..150)).map(|_| rng.sample(Alphanumeric) as char).collect(),
            port: 8081,
            host: "127.0.0.1".to_string(),
            users: vec![
                User {
                    login: String::from("Admin"),
                    password: String::from("Admin"),
                    path: String::from("/"),
                    limit: 1024,
                    admin: true,
                }
            ],
        };
        let config_path = Path::new("./config.yaml");
        if !config_path.exists() {
            let mut file = File::create(config_path)?;
            file.write_all(serde_yaml::to_string(&default)?.as_bytes())?;
            println!("Config initialized with default values!");
            Ok(default)
        } else {
            let mut file = File::open(config_path)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            let structure = serde_yaml::from_str::<Config>(&content)?;
            println!("Config loaded from file!");
            Ok(structure)
        }
    }
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::open("./config.yaml")?;
        file.write_all(serde_yaml::to_string(self)?.as_bytes())?;
        Ok(())
    }
    pub fn get_host_address(&self) -> (String, u16) {
        (self.host.clone(), self.port)
    }
    pub fn create_db(&self) -> DataBase {
        DataBase {
            users: Mutex::new(self.users.clone())
        }
    }
}

#[derive(Debug)]
pub struct DataBase {
    users: Mutex<Vec<User>>,
}

impl DataBase {
    pub fn find_user_by_login(&self, login: &str) -> SResult<Option<User>> {
        let users = self.users.lock().unwrap();
        for user in users.iter() {
            if user.login.eq_ignore_ascii_case(login) {
                return Ok(Some(user.clone()));
            }
        }
        Ok(None)
    }
}

impl Clone for DataBase {
    fn clone(&self) -> Self {
        Self {
            users: Mutex::new(self.users.lock().unwrap().clone())
        }
    }
}