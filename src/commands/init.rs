use clap::ValueEnum;

#[derive(Debug,Clone,ValueEnum)]
pub enum Templates {
    Empty,
    HelloWorld
}

pub fn init(name: String, template: Templates) {
    println!("Initiating {name}, with template {template:?}")
}