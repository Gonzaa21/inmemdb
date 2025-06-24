# InMemDB
An in-memory database engine written in Rust, inspired by Redis.
- Support for commands like::
  - `SET key value`
  - `GET key`
  - `DEL key`

## Use
1. Clone & compile
```bash
    git clone https://github.com/tu-usuario/inmemdb.git
    cd inmemdb
    cargo build --release
```
2. Server online
```bash
    cargo run
```
3. Connect with Telnet or Netcat
```bash
    telnet 127.0.0.1 6379
```
```bash
    nc 127.0.0.1 6379
```
## Structure
```yaml
inmemdb/
├── Cargo.toml
└── src/
    ├── main.rs              # Tokio runtime
    ├── server.rs            # accept conections and parse commands
    ├── db.rs                # database
    ├── commands/            # command register
```