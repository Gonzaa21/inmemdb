```yaml
inmemdb/
├── Cargo.toml
└── src/
    ├── main.rs              # Tokio runtime
    ├── server.rs            # accept conections and parse commands
    ├── db.rs                # database
    ├── commands/            # command register
```