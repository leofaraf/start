use std::{error::Error, time::Instant};

use bson::doc;
use serde::{Deserialize, Serialize};
use start::db::query::filtering::{Filter, Value};

type HandleResult<T> = Result<T, Box<dyn Error>>;

#[derive(Serialize, Deserialize, Debug)]
struct Agent {
    name: String,
    r#type: String,
    score: i32,
}

fn main() -> HandleResult<()> {
    let start = Instant::now();

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
    
    session.update()
        .filter(Filter::Eq("name".into(), Value::String("Cloude".into())))
        .set(doc! {
            "score": 90
        })
        .from("american-ai")?;

    // session.delete()
    //     .filter(Filter::Gt("score".into(), Value::Integer(85)))
    //     .from("american-ai")?;

    let result: Vec<Agent> = session.find()
        .filter(Filter::Gt("score".into(), Value::Integer(85)))
        .limit(1)
        .from("american-ai")?;

    println!("----Collection-----");
    
    for entry in result {
        println!("Entry: {:?}", entry);
    }

    println!("-------------------");

    session.rollback_transaction();

    let result: Vec<Agent> = session.find()
        .from("american-ai")?;
    
    println!("--AfterCollection--");
    
    for entry in result {
        println!("Entry: {:?}", entry);
    }
    
    println!("-------------------");

    println!("Main took: {}ms", start.elapsed().as_millis());
    Ok(())
}
