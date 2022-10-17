use serde::Serialize;
use chrono::{DateTime, Utc};
use tinytemplate::TinyTemplate;
use std::error::Error;

#[derive(Serialize)]
struct Context {
    name: String,
}

static NOW_TIME_TEMPLATE: &'static str = "{name}";

pub fn main() {
    let time = time_render();
    println!("{:?}", time)
}

fn time_render() -> Result<(), Box<dyn Error>> {
    let now: DateTime<Utc> = Utc::now();
    let now_formated = now.format("%Y-%m-%dT%H:%M:%S");


    let mut tt = TinyTemplate::new();
    tt.add_template("hello", NOW_TIME_TEMPLATE)?;

    let context = Context {
        name: now_formated.to_string(),
    };

    let rendered = tt.render("hello", &context)?;
    println!("{}", rendered);

    Ok(())
}