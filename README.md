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

## Roadmap

### Operations

1. Find
- [X] Basic execution
- [ ] Handle filtering, limiting, etc.
- [ ] Indexing (B+tree)
- [ ] Mongo-style cursor 

2. Insert
- [X] Simple one-doc insert
- [ ] Ids
- [ ] Indexing (B+tree)

3. Update
- [ ] Simple bulk update by filter

4. Delete
- [ ] Delete operation by filters

### ACID

Now, impliment AC__ (atomicity, consistency, isolation, durability).

1. Atomicity
- [X] Basic functional for all operations (RecoveryUnit)
- [X] Session catalog (TransactionParticipant -> txnNumber, recovery unit (changes) )
- [X] Multi-document transactions (based on session catalog)
- [X] Refactor Recovery Unit
- [X] Fully “All or Nothing” atomocity

2. Consistency
- [X] Develop new: Inconsistent because of "collection catalog"
- [ ] Validation hooks or constraints during commit

3. Isolation
- [ ] Locks

4. Durability
- [ ] WAL (Write Ahead Log)
- [ ] Crash recovery on startup

### Rest

1. More efficent Space managment system
- [ ] Trash (_system_trash)
- [ ] Storage pages like in mongo

2. Redesign filters? (mb include limits etc. inside)

3. SQL (startdb query language) parsing

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
