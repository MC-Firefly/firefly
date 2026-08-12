use clap::{Parser,Subcommand};
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

static VERSION_MAJOR: &str = "0";
static VERSION_MINOR: &str = "0";
static VERSION_PATCH: &str = "1";
static VERSION_ADDENDUM: &str = "-infdev";


fn main() {
    let args = Cli::parse();

    match args.command {
        Subcommands::Init { name, template } => {
            commands::init::init(name,template);
        }
        Subcommands::Build => {
            todo!("Build subcommand");
        }
        Subcommands::Version => {
            println!("Running Firefly v{VERSION_MAJOR}.{VERSION_MINOR}.{VERSION_PATCH}{VERSION_ADDENDUM}");
        }
    }
}
