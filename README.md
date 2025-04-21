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
    let mut db = start::in_memory();

    db
        .insert(Agent {
            name: "ChatGPT".into(),
            r#type: "AI".into(),
            score: 85
        })
        .into("agents")?;

    db
        .insert(Agent {
            name: "Gemini".into(),
            r#type: "AI".into(),
            score: 80
        })
        .into("agents")?;

    db
        .insert(Agent {
            name: "RuleBot3000".into(),
            r#type: "Rule-Based".into(),
            score: 100
        })
        .into("agents")?;

    let many: Vec<Agent> = db.find()
        .filter(Filter::And(vec![
            Filter::Eq("type".into(), Value::String("AI".into())),
            Filter::Gt("score".into(), Value::Integer(80)),
        ]))
        .from("agents")?;

    // output:
    // Agent { name: "ChatGPT", type: "AI", score: 85 }

    for doc in many {
        println!("{:?}", doc);
    }

    Ok(())
}
```

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
- [ ] limit
- [ ] insertMany
- [ ] storage-pages
- [ ] delete
- [ ] make lazy colscan if collection catalog lookup don't find
- [ ] session catalog (TransactionParticipant -> txnNumber, recovery unit (changes) )

## How does it works?

Based on `start-storage` crate, database first keeps 100 bytes header.

Next it (sys-master) contains tables, first system-tables (like sys-master, then sys-trash)

At second, it keeps user tables. Each table is linked list.

##### TO REMEMBER

redesign dbs - collection-master doc-next key as next document, doc-content-next as next collection
