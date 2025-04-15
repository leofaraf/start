# IN DEVELOPMENT DON'T USE IN PRODUCTION (wait stabilization)

* database in development and might rapidly change syntax

in_memory/embedded (single file) database

### Example of using in code

```rust
let mut db = start::in_memory();

#[derive(Serialize, Deserialize, Debug)]
struct Agent {
    name: String,
    r#type: String,
}

db
    .insert(Agent {
        name: "ChatGPT".into(),
        r#type: "AI".into(),
    })
    .into("agents")?;

db
    .insert(Agent {
        name: "Cloude".into(),
        r#type: "AI".into(),
    })
    .into("agents")?;

let many: Vec<Agent> = db.find().from("agents")?;
for doc in many {
    println!("{:?}", doc);
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
find({args}) [ ]
storage-pages [ ]
limit [ ]
insertMany [ ]
bson [ ]
delete [ ]

## How does it works?

Based on `start-storage` crate, database first keeps 100 bytes header.

Next it (sys-master) contains tables, first system-tables (like sys-master, then sys-trash)

At second, it keeps user tables. Each table is linked list.