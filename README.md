# IN DEVELOPMENT DON'T USE IN PRODUCTION (wait stabilization)

* database in development and might rapidly change syntax

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
    let ctx = start::service_context::in_memory();

    commands::insert::insert(&ctx, "american-ai", bson::to_bson(&Agent {
        name: "ChatGPT".to_string(),
        r#type: "AI".to_string(),
        score: 85,
    }).unwrap());

    commands::insert::insert(&ctx, "chinese-ai", bson::to_bson(&Agent {
        name: "DeepSeek".to_string(),
        r#type: "AI".to_string(),
        score: 80,
    }).unwrap());
    
    commands::insert::insert(&ctx, "american-ai", bson::to_bson(&Agent {
        name: "Cloude".to_string(),
        r#type: "AI".to_string(),
        score: 85,
    }).unwrap());

    let result = commands::find::find(
        &ctx,
        "american-ai",
        None, None, None
    );

    println!("----Collection-----");
    
    for entry in result {
        println!("Entry: {:?}", entry);
    }

    println!("-------------------");

    // output:
    // ----Collection-----
    // Entry: Document({"name": String("ChatGPT"), "type": String("AI"), "score": Int32(85)})
    // Entry: Document({"name": String("Cloude"), "type": String("AI"), "score": Int32(85)})
    // -------------------
    Ok(())
}
```

## Current state

Now, impliment A__D (atomicity, consistency, isolation, durability).

### quick roadmap:

documnet [x]
collection [x]
findCollection [x]
insertCollectionByOffset [x]
insertOne [x]
insertCollection [x]
insertDML [x]
find [x]
findDML [x]
find({args}) [x]

- [X] bson
- [X] Atomicity
- [X] Durability
- [ ] session catalog (TransactionParticipant -> txnNumber, recovery unit (changes) )
- [ ] Consistency (almost, check about collection catalog)
- [ ] Isolation
- [ ] limit
- [ ] insertMany
- [ ] storage-pages
- [ ] delete
- [ ] make lazy col. check if collection catalog lookup don't find
- [ ] indexes
- [ ] SQL parsing
- [ ] Concurency (RwLock)

## How does it works?

Based on `start-storage` crate, database first keeps 100 bytes header.

Next it (sys-master) contains tables, first system-tables (like sys-master, then sys-trash)

At second, it keeps user tables. Each table is linked list.

##### TO REMEMBER

redesign dbs - collection-master doc-next key as next document, doc-content-next as next collection
