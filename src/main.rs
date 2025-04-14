use std::error::Error;

use serde::Serialize;
use start::{systypes::document::RawDocument, sysutils::find::{find_collection::find_collection, scan::scan}, utils::insert::insert_one};

type HandleResult<T> = Result<T, Box<dyn Error>>;

#[derive(Serialize)]
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

    // insert_one(&mut db.ss, "students", "leon".as_bytes().to_vec());
    let students = find_collection(&mut db.ss, "agents");
    if let Some(table) = students {
        scan(&mut db.ss, table);
    }
    println!("students col: {:?}", students);

    Ok(())
}
