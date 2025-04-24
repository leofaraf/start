# IN DEVELOPMENT DON'T USE IN PRODUCTION (wait stabilization)

* database in development and might rapidly change syntax

# StartDB

in_memory/embedded (single file) database

### Example of using in code

```rust
#[derive(Serialize, Deserialize, Debug)]
struct Agent {
    name: String,
    r#type: String,
    score: i32,
}

fn main() -> HandleResult<()> {
    let ctx = service_context::in_memory();
    let db = StartDB {
        ctx,
    };

    let session = db.get_session();

    session.start_transaction();

    commands::insert::insert(&session, "american-ai", bson::to_bson(&Agent {
        name: "ChatGPT".to_string(),
        r#type: "AI".to_string(),
        score: 85,
    }).unwrap());

    commands::insert::insert(&session, "chinese-ai", bson::to_bson(&Agent {
        name: "DeepSeek".to_string(),
        r#type: "AI".to_string(),
        score: 80,
    }).unwrap());
    
    commands::insert::insert(&session, "american-ai", bson::to_bson(&Agent {
        name: "Cloude".to_string(),
        r#type: "AI".to_string(),
        score: 85,
    }).unwrap());

    let result = commands::find::find(
        &session,
        "american-ai",
        None, None, None
    );

    println!("----Collection-----");
    
    for entry in result {
        println!("Entry: {:?}", entry);
    }

    println!("-------------------");

    session.commit_transaction();

    // output:
    // ----Collection-----
    // Entry: Document({"name": String("ChatGPT"), "type": String("AI"), "score": Int32(85)})
    // Entry: Document({"name": String("Cloude"), "type": String("AI"), "score": Int32(85)})
    // -------------------
    Ok(())
}
```

## How does it works?

Based on `start-storage` crate, database first keeps 100 bytes header.

Next it (sys-master) contains tables, first system-tables (like sys-master, then sys-trash)

At second, it keeps user tables. Each table is linked list.

##### TO REMEMBER

redesign dbs - collection-master doc-next key as next document, doc-content-next as next collection
