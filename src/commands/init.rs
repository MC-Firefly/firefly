use std::fs::{create_dir, File};
use std::io;
use std::io::Write;
use clap::ValueEnum;

#[derive(Debug,Clone,ValueEnum)]
pub enum Templates {
    Empty,
    HelloWorld
}

pub fn init(name: String, template: Templates) -> io::Result<()> {
    println!("Initiating {name}, with template {template:?}");
    create_dir(&name)?;
    create_dir(format!("{name}/src"))?;
    File::create(format!("{name}/config.toml"))?;
    if matches!(template, Templates::HelloWorld) {
        let mut template = File::create(format!("{name}/src/main.ff"))?;
        template.write_all(br#"tellraw @a "Hello World!""#)?;
    }
    Ok(())
}