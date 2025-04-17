use std::error::Error;

use serde::{Deserialize, Serialize};
use start::{query_builder::filtering::{value::Value, Filter}, utils::find::find_many};

type HandleResult<T> = Result<T, Box<dyn Error>>;

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
