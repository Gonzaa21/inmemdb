use crate::config::Config;
use crate::db::Database;
use crate::command::{parse_command, Command};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::RwLock;
use std::io::Result;
use std::net::SocketAddr;
use std::collections::HashMap;
use std::sync::Mutex;

pub async fn run(db: Arc<RwLock<Database>>, _password: String) -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    let auth_map = Arc::new(Mutex::new(HashMap::<SocketAddr, bool>::new()));
    loop {
        let (socket, addr) = listener.accept().await?; // accept TCP conection
        let db = db.clone(); // clone Arc
        let auth_map = auth_map.clone();

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

                let is_authed = {
                    let map = auth_map.lock().unwrap(); // block access
                    *map.get(&addr).unwrap_or(&false) // verify client state
                };

                match parse_command(line.trim()) {
                    Ok(command) => {
                        if !is_authed && !matches!(command, Command::Auth(_)) {
                                writer.write_all(b"-NOAUTH Authentication required\r\n").await.unwrap();
                                continue;
                        }
                        let response = match command {
                            Command::Set(key, value) => {
                                let mut db = db.write().await;
                                db.set(key, value);
                                "+OK\r\n".to_string()
                            }
                        
                            Command::Get(key) => {
                                let db = db.read().await;
                                match db.get(&key) {
                                    Some(value) => format!("${}\r\n{}\r\n", value.len(), value),
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

                            Command::Exists(key) => {
                                let mut db = db.write().await;
                                if db.exists(&key) {
                                    ":1\r\n".to_string()
                                } else {
                                    ":0\r\n".to_string()
                                }
                            }

                            Command::Incr(key) => {
                                let mut db = db.write().await;
                                let new_val = match db.get(&key) {
                                    Some(s) => match s.parse::<i64>() {
                                        Ok(n) => n + 1,
                                        Err(_) => {
                                            writer.write_all(b"-ERR value is not an integer\r\n").await.unwrap();
                                            continue;
                                        }
                                    },
                                    None => 1,
                                };
                                db.set(key, new_val.to_string());
                                format!(":{}\r\n", new_val)
                            }

                            Command::Flush => {
                                let mut db = db.write().await;
                                db.flush();
                                "+OK\r\n".to_string()
                            }

                            Command::Scan => {
                                let db = db.read().await;
                                let keys = db.scan();
                                if keys.is_empty() {
                                    "*0\r\n".to_string()
                                } else {
                                    let mut response = format!("*{}\r\n", keys.len());
                                    for k in keys {
                                        response.push_str(&format!("${}\r\n{}\r\n", k.len(), k));
                                    }
                                    response
                                }
                            }

                            Command::Ping => {
                                "+PONG\r\n".to_string()
                            }

                            Command::Echo(key) => {
                                format!("${}\r\n", key)
                            }

                            Command::Save => {
                                let db = db.read().await;
                                match db.save() {
                                    Ok(()) => "+OK\r\n".to_string(),
                                    Err(_) => "-ERR Could not save\r\n".to_string()
                                }
                            }

                            Command::Auth(password) => {
                                let config = Config::load_file("config.json").expect("Failed to load config");
                                let config_password = config.password;
                                if password == config_password {
                                    // update map with the auth
                                    let mut map = auth_map.lock().unwrap();
                                    map.insert(addr, true);
                                    "+OK\r\n".to_string()
                                } else {
                                    "-ERR invalid password\r\n".to_string()
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