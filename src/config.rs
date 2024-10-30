use dotenv::dotenv;
use std::env;

pub fn load_env() {
    dotenv().ok();
    if env::var("DATABASE_URL").is_err() {
        panic!("DATABASE_URL must be set");
    }
}