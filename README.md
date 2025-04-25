# IN DEVELOPMENT DON'T USE IN PRODUCTION (wait stabilization)

* database in development and might rapidly change syntax

# StartDB

in_memory/embedded (single file) database

# Features

- Simple API ✨.
- Transparent serialization/deserialization using `serde`.
- ACID-compliant, transaction (Mongo-Inspired).
- Powerful querying system (`filters`, `limit`, `skip` and aggregation is in development)
- It's own SQL dialect soon

# Installation

Add this to your `Cargo.toml`:
```toml
[dependencies]
start = "0.2.0"
```

# Status

Early stage project — the API is experimental and subject to change.

# How to use?

- [Quick Start](docs/QUICK_START.md)
- [Documentation API](https://docs.rs/start)

# Example
```rust
use serde::{Deserialize, Serialize};
use start::db::query::filtering::{Filter, Value};

type HandleResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Serialize, Deserialize, Debug)]
struct Agent {
    name: String,
    r#type: String,
    score: i32,
}

fn main() -> HandleResult<()> {
    let db = start::db_in_memory()?;
    let session = db.get_session();

    session.start_transaction()?;
    
    session.insert("agents", 
        &Agent {name: "Cloude".to_string(), r#type: "AI".to_string(), score: 88})?;
    session.insert("agents",
        &Agent {name: "ChatGPT".to_string(), r#type: "AI".to_string(), score: 90})?;
    session.insert("agents",
        &Agent {name: "Gemini".to_string(), r#type: "AI".to_string(), score: 85})?;

    let result: Vec<Agent> = session.find()
        .filter(Filter::Gt("score".into(), Value::Integer(85)))
        .from("agents")?;
    for entry in result {
        println!("Entry: {:?}", entry);
    }

    // Entry: Agent { name: "Cloude", type: "AI", score: 88 }
    // Entry: Agent { name: "ChatGPT", type: "AI", score: 90 }
    session.commit_transaction()?;
    Ok(())
}
```