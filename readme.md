```yaml
inmemdb/
│
├── Cargo.toml
└── src/
    ├── main.rs              # Servidor TCP + Tokio runtime
    ├── server.rs            # Acepta conexiones y parsea comandos
    ├── db.rs                # Motor de base de datos en memoria
    ├── commands/
    │   ├── mod.rs           # Registro de todos los comandos
    │   ├── get.rs
    │   ├── set.rs
    │   └── del.rs
    ├── types.rs             # Tipos genéricos y alias comunes
    └── utils.rs             # Logging, parsing, helpers
```