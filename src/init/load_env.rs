use dotenv::dotenv;
use std::env;

pub fn load_env() {
    // Определяем среду: development или production
    let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "DEVELOPMENT".to_string());

    match environment.as_str() {
        "PRODUCTION" => {
            println!("Running in PRODUCTION mode");
        }
        _ => {
            println!("Running in DEVELOPMENT mode");
            dotenv().ok();
        }
    }
}
