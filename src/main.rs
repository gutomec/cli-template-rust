use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cli-template")]
#[command(about = "Production CLI template with Clap", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Say hello
    Hello {
        /// Name to greet
        #[arg(short, long, default_value = "World")]
        name: String,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Hello { name } => {
            println!("Hello, {}!", name);
        }
    }
}
