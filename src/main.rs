use clap::{Parser,Subcommand};
mod commands;

#[derive(Subcommand)]
enum Subcommands {
    Init {
        name: String,

        #[arg(short='t',long="template",value_enum,default_value_t=commands::init::Templates::Empty)]
        template: commands::init::Templates
    },
    Build
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Subcommands
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Subcommands::Init { name, template } => {
            commands::init::init(name,template);
        }
        Subcommands::Build => {
            todo!("Build subcommand")
        }
    }
}
