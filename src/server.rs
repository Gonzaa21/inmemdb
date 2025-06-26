use crate::db::Database;
use crate::command::{parse_command, Command};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::RwLock;
use std::io::Result;

pub async fn run(db: Arc<RwLock<Database>>) -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    loop {
        let (socket, addr) = listener.accept().await?; // accept TCP conection
        let db = db.clone(); // clone Arc

        println!("Client connected: {}", addr); // debug addres

        tokio::spawn(async move {
            let (reader, mut writer) = socket.into_split();
            let mut reader = BufReader::new(reader);

            let mut line = String::new();
            loop {
                line.clear(); // clean buffer
                let bytes_read = match reader.read_line(&mut line).await {
                    Ok(n) => n,
                    Err(_) => break
                }; // read each line and manage errors

                if bytes_read == 0 {break} // if not have nothing => cancel

                match parse_command(line.trim()) {
                    Ok(command) => {
                        let response = match command {
                            Command::Set(key, value) => {
                                let mut db = db.write().await;
                                db.set(key, value);
                                "+OK\r\n".to_string()
                            }
                        
                            Command::Get(key) => {
                                let db = db.read().await;
                                match db.get(&key) {
                                    Some(value) => format!("${}\r\n", value),
                                    None => "$nil\r\n".to_string(),
                                }
                            }
                        
                            Command::Del(key) => {
                                let mut db = db.write().await;
                                if db.del(&key) {
                                    "+OK\r\n".to_string()
                                } else {
                                    "-ERR key not found\r\n".to_string()
                                }
                            }
                        };

                        writer.write_all(response.as_bytes()).await.unwrap();
                    }
                    Err(e) => { // print error in console and continue runing
                        let err_msg = format!("-ERR {}\r\n", e);
                        writer.write_all(err_msg.as_bytes()).await.unwrap();
                        continue;
                    }
                }
            }
        });
    }
}