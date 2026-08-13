use std::fs::{create_dir, File, exists, read_dir};
use std::io;
use std::io::Write;
use clap::ValueEnum;
use std::process::exit;

#[derive(Debug,Clone,ValueEnum)]
pub enum Templates {
    Empty,
    HelloWorld
}

pub fn init(name: String, template: Templates) -> anyhow::Result<()> {
    if exists(&name)? && read_dir(&name)?.next().is_some() {
        println!("Error initializing: folder is not empty!");
        exit(65) // Run cat /usr/include/sysexits.h on your system. EX_DATAERR.
    }

    println!("Initiating {name}, with template {template:?}");

    create_dir(&name)?;
    create_dir(format!("{name}/src"))?;

    let mut config = File::create(format!("{name}/firefly.toml"))?;
    config.write(toml::to_string(&crate::Config::default())?.as_bytes())?;

    match template {
        Templates::Empty => {
            File::create(format!("{name}/src/main.ff"))?;
        }
        Templates::HelloWorld => {
            let mut template = File::create(format!("{name}/src/main.rs"))?;
            template.write_all(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/HelloWorld.ff")))?;
        }
    }
    
    Ok(())
}