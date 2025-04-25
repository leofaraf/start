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
    let db = start::db_embedded("file.db".into())?;
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

    // Content written in file.db
    // Entry: Agent { name: "Cloude", type: "AI", score: 88 }
    // Entry: Agent { name: "ChatGPT", type: "AI", score: 90 }
    session.commit_transaction()?;
    Ok(())
}