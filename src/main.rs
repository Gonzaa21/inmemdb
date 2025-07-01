mod db;
mod server;
mod command;
mod error;
use db::Database;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    env_logger::init();
    let db = Arc::new(RwLock::new(Database::new()));
    server::run(db).await.unwrap();
    // debug
    // let cmd = command::parse_command("SET key value");
    // print!("{:?}",cmd);
}

// // Tareas:
// SAVE
// sistema thiserror
// auth y establecer passwords