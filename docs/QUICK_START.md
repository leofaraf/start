# Quick start

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
    // Start an in-memory database instance
    let db = start::db_in_memory()?;
    let session = db.get_session();

    session.start_transaction()?;

    // INSERT
    session.insert("agents", &Agent {
        name: "Cloude".into(),
        r#type: "AI".into(),
        score: 88,
    })?;

    session.insert("agents", &Agent {
        name: "ChatGPT".into(),
        r#type: "AI".into(),
        score: 90,
    })?;

    session.insert("agents", &Agent {
        name: "Gemini".into(),
        r#type: "AI".into(),
        score: 85,
    })?;

    // UPDATE
    use bson::doc;
    session.update()
        .filter(Filter::Eq("name".into(), Value::String("Cloude".into())))
        .set(doc! {
            "score": 91
        })
        .from("agents")?;

    // DELETE
    session.delete()
        .filter(Filter::Lt("score".into(), Value::Integer(86)))
        .from("agents")?;

    // FIND
    let result: Vec<Agent> = session.find()
        .filter(Filter::Gt("score".into(), Value::Integer(85)))
        .from("agents")?;

    for agent in result {
        println!("Found Agent: {:?}", agent);
    }

    session.commit_transaction()?;
    Ok(())
}
```

## What else?

Look at [examples](../examples/)

## 💡 About In-Memory/Embedded Mode
StartDB supports running entirely in-memory or using a single embedded file as persistent storage. This means:

⚡ Zero external dependencies or database servers.

📁 Data is stored in memory (or a file if you choose) — perfect for bots, testing environments, or CLI tools.

🔒 Full transaction support even in embedded mode.