use std::{error::Error, time::Instant};

use serde::{Deserialize, Serialize};
use start::query_builder::filtering::{value::Value, Filter};

type HandleResult<T> = Result<T, Box<dyn Error>>;

#[derive(Serialize, Deserialize, Debug)]
struct Agent {
    name: String,
    r#type: String,
    score: i32,
}

fn main() -> HandleResult<()> {
    let start = Instant::now();
    let inst = start;
    let mut db = start::in_memory();
    println!("Db inizialization took: {}ms", inst.elapsed().as_millis());

    let inst = Instant::now();
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
    println!("Inserting took: {}ms", inst.elapsed().as_millis());

    let inst = Instant::now();
    let many: Vec<Agent> = db.find()
        .filter(Filter::And(vec![
            Filter::Eq("type".into(), Value::String("AI".into())),
            Filter::Gt("score".into(), Value::Integer(80)),
        ]))
        .from("agents")?;
    println!("Filtering took: {}ms", inst.elapsed().as_millis());

    // output:
    // Agent { name: "ChatGPT", type: "AI", score: 85 }

    for doc in many {
        println!("{:?}", doc);
    }

    println!("Main took: {}ms", start.elapsed().as_millis());

    Ok(())
}
