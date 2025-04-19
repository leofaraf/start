use std::{error::Error, time::Instant};

use serde::{Deserialize, Serialize};
use start::{db::{commands, service_context::{self, ServiceContext}}, query_builder::filtering::{value::Value, Filter}};

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

    let result = commands::find::find(
        &ctx,
        "students",
        None, None, None
    );

    println!("result: {:?}", result);

    println!("Main took: {}ms", start.elapsed().as_millis());
    Ok(())
}
