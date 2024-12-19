use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub mongo_uri: String,
}

pub fn init() -> Config {
    let panic_message: String = "enviroment variable is not set".to_string();

    Config {
        mongo_uri: match env::var("MONGO_URI") {
            Ok(var) => var,
            Err(_) => panic!("MONGO_URI {}", panic_message),
        },
    }
}
