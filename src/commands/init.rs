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
    if fs::exists(&name)? && fs::read_dir(&name)?.next().is_some() {
        println!("Error initializing: folder is not empty!");
        exit(65) // Run cat /usr/include/sysexits.h on your system. EX_DATAERR.
    }

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