use std::fs::File;
use std::io::Write;
use datavzrd_lib::ItemsSpec;
use schemars::schema_for;
use tera::{Context, Tera};
use anyhow::Result;

fn main() -> Result<()>{
    let schema = schema_for!(ItemsSpec);
    let mut context = Context::new();
    context.insert("schema", &serde_json::to_string_pretty(&schema)?);
    let f = Tera::one_off(include_str!("../templates/schemavzrd.html.tera"), &context, true)?;
    let mut file = File::create("schemavzrd.html")?;
    file.write_all(&f.as_bytes())?;
    Ok(())
}