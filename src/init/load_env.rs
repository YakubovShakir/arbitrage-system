use dotenv::dotenv;
use log::info;
use std::env;

pub fn load_env() {
    // Определяем среду: development или production
    let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "DEVELOPMENT".to_string());

    match environment.as_str() {
        "PRODUCTION" => {
            info!(target: "info_module", "Running in PRODUCTION mode");
        }
        _ => {
            info!(target: "info_module", "Running in DEVELOPMENT mode");
            dotenv().ok();
        }
    }
}
