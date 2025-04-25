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

Active development. The API is not stable yet and may change in the future.

# How to use?

- [Documentation API](doc)
- [Quick Start](quick-start)

# Example
```rust
#[derive(Serialize, Deserialize, Debug)]
struct Agent {
    name: String,
    r#type: String,
    score: i32,
}

fn main() -> HandleResult<()> {
    let db = start::db_in_memory();
    let session = db.get_session();

    session.start_transaction();
    
    session.insert("american-ai", &Agent {
        name: "Cloude".to_string(),
        r#type: "AI".to_string(),
        score: 85,
    })?;

    session.insert("american-ai", &Agent {
        name: "ChatGPT".to_string(),
        r#type: "AI".to_string(),
        score: 90,
    })?;

    session.insert("american-ai", &Agent {
        name: "Gemini".to_string(),
        r#type: "AI".to_string(),
        score: 80,
    })?;

    session.insert("chinese-ai", &Agent {
        name: "DeepSeek".to_string(),
        r#type: "AI".to_string(),
        score: 85,
    })?;
    
    session.delete()
        .filter(Some(Filter::Gt("score".into(), Value::Integer(85))))
        .from("american-ai")?;

    let result: Vec<Agent> = session.find()
        .from("american-ai")?;

    println!("----Collection-----");
    
    for entry in result {
        println!("Entry: {:?}", entry);
    }

    println!("-------------------");

    // output:
    // ----Collection-----
    // Entry: Agent { name: "Cloude", type: "AI", score: 85 }
    // Entry: Agent { name: "Gemini", type: "AI", score: 80 }
    // -------------------
    Ok(())
}
```

## How does it works?

Based on `start-storage` crate, database first keeps 100 bytes header.

Next it (sys-master) contains tables, first system-tables (like sys-master, then sys-trash)

At second, it keeps user tables. Each table is linked list.