# StartDB

[![Crates.io](https://img.shields.io/crates/v/start)](https://crates.io/crates/start)
[![Documentation](https://docs.rs/start/badge.svg)](https://docs.rs/start)
![GitHub License](https://img.shields.io/github/license/leofaraf/start)

A lightweight, in-memory/embedded (single-file) ACID-compliant database designed for simplicity and flexibility.

P.S. don't use 'update' for now, working on optimization

# Features

- Simple API ✨.
- Embedded & In-Memory: Data lives in memory or a single file—ideal for CLI tools, bots, or lightweight services.
- ACID Transactions: Mongo-inspired session-based transactions.
- Powerful Querying: Supports filter, limit, skip (aggregations and joins coming soon).
- Serde Support: Transparent serialization and deserialization using serde.
- Experimental SQL dialect planned 🧪.

# Installation

Add this to your `Cargo.toml`:
```toml
[dependencies]
start = "0.4"
```

# Status

Early stage project — the API is experimental and subject to change.

# How to use?

- [Quick Start](https://github.com/leofaraf/start/blob/master/docs/QUICK_START.md)
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
