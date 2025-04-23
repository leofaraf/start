use std::{error::Error, rc::Rc, time::Instant};

use serde::{Deserialize, Serialize};
use start::{db::{commands, operation_context::OperationContext, service_context}, StartDB};

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

    session.rollback_transaction();

    let result = commands::find::find(
        &session,
        "american-ai",
        None, None, None
    );
    
    println!("--AfterCollection--");
    
    for entry in result {
        println!("Entry: {:?}", entry);
    }
    
    println!("-------------------");

    println!("Main took: {}ms", start.elapsed().as_millis());
    Ok(())
}
