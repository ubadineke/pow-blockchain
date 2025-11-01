use balances::manage_balances;
use clap::{Parser, Subcommand, command};
use tx::manage_txs;
pub mod balances;
pub mod tx;

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
    #[command(about = "Manage transactions")]
    Tx {
        #[command(subcommand)]
        command: TxCommands,
    }
}

#[derive(Debug, Subcommand)]
pub enum BalancesCommands {
    #[command(about = "list account balances")]
    List,
}

#[derive(Debug, Subcommand)]
pub enum TxCommands {
    #[command(about = "add transactions")]
    Add(AddArgs)
}

#[derive(Parser, Debug)]
pub struct AddArgs{
    #[arg(long, help = "From what account to send tokens")]
    from: String,

    #[arg(long, help = "To what account to send tokens")]
    to: String,

    #[arg(long, help = "How many tokens to send")]
    value: u64,

    #[arg(long, help = "Is this a reward transaction")]
    data: Option<String>

}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Balances { command } => manage_balances(command),
        Commands::Tx { command } => manage_txs(command,),
    }
}
