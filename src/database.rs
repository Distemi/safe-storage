use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

#[derive(Deserialize, Serialize, Debug)]
pub struct DataBase {
    pub cookie_key: String,
    pub port: u16,
    pub host: String,
    pub users: Vec<User>,
}

impl DataBase {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let mut rng = thread_rng();
        let default = DataBase {
            cookie_key: (0..50).map(|_| rng.sample(Alphanumeric) as char).collect(),
            port: 8081,
            host: "127.0.0.1".to_string(),
            users: vec![],
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
            let structure = serde_yaml::from_str::<DataBase>(&content)?;
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
    pub fn find_user_by_login(&self, login: &str) -> Option<&User> {
        for user in &self.users {
            if user.login.eq_ignore_ascii_case(login) {
                return Some(user);
            }
        }
        None
    }
}

impl Clone for DataBase {
    fn clone(&self) -> Self {
        Self {
            cookie_key: "".to_string(),
            host: String::new(),
            port: 0,
            users: self.users.clone(),
        }
    }
}

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