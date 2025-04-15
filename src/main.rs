use std::error::Error;

use serde::{Deserialize, Serialize};
use start::utils::find::find_many;

type HandleResult<T> = Result<T, Box<dyn Error>>;

#[derive(Serialize, Deserialize, Debug)]
struct Agent {
    name: String,
    r#type: String,
}

fn main() -> HandleResult<()> {
    let mut db = start::in_memory();

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

    // insert_one(&mut db.ss, "students", "leon".as_bytes().to_vec());
    // let many = find_many(&mut db.ss, "agents");
    for doc in many {
        println!("{:?}", doc);
    }

    Ok(())
}
