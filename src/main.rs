use std::{error::Error, time::Instant};

use serde::{Deserialize, Serialize};
use start::db::{commands, service_context};

type HandleResult<T> = Result<T, Box<dyn Error>>;

#[derive(Serialize, Deserialize, Debug)]
struct Agent {
    name: String,
    r#type: String,
    score: i32,
}

fn main() -> HandleResult<()> {
    let start = Instant::now();

    let ctx = service_context::in_memory();

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

    println!("Main took: {}ms", start.elapsed().as_millis());
    Ok(())
}
