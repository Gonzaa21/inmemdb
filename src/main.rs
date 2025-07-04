mod db;
mod server;
mod command;
mod error;
mod config;
use crate::config::Config;
use db::Database;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    env_logger::init();
    let db = Arc::new(RwLock::new(Database::new()));
    let config = Config::load_file("config.json")
        .expect("The config file could not be loaded");

    server::run(db, config.password).await.unwrap();
    // debug
    // let cmd = command::parse_command("SET key value");
    // print!("{:?}",cmd);
}

// // Tareas:
// auth y establecer passwords