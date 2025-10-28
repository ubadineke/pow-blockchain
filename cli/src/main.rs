use balances::manage_balances;
use clap::{Parser, Subcommand, command};
pub mod balances;

#[derive(Parser, Debug)]
#[command(name = "ubachain")]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Manage balances")]
    Balances {
        #[command(subcommand)]
        command: BalancesCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum BalancesCommands {
    #[command(about = "list account balances")]
    List,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Balances { command } => manage_balances(command),
    }
}
