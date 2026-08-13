use clap::{Parser,Subcommand};
use serde::{Deserialize, Serialize};

mod commands;

#[derive(Subcommand)]
enum Subcommands {
    Init {
        name: String,

        #[arg(short='t',long="template",value_enum,default_value_t=commands::init::Templates::Empty)]
        template: commands::init::Templates
    },
    Build,
    Version
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Subcommands
}

#[derive(Deserialize,Serialize)]
pub struct Config<'a> {
    default_namespace: &'a str,
    target_version: &'a str,
}

impl Default for Config<'_> {
    fn default() -> Self {
        Self { default_namespace: "firefly", target_version: "26.2" }
    }
}

static VERSION: (u32, u32, u32, &str) = (0, 0, 1, "-infdev");


fn main() {
    let args = Cli::parse();

    match args.command {
        Subcommands::Init { name, template } => {
            if let Err(e) = commands::init::init(name,template) {
                eprintln!("Error: {}", e);
            }
        }
        Subcommands::Build => {
            if let Err(e) = commands::build::build() {
                eprintln!("Error: {}", e);
            }
        }
        Subcommands::Version => {
            let (major, minor, patch, suffix) = VERSION;
            println!("Running Firefly v{}.{}.{}{}", major, minor, patch, suffix);
        }
    }
}
