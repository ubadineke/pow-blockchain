use balances::manage_balances;
use clap::{Parser, Subcommand};
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
    /// Manage balances
    Balances {
        #[command(subcommand)]
        command: BalancesSubcommand,
    },

    /// Manage transactions
    Tx {
        #[command(subcommand)]
        command: TxSubcommand,
    },
}

#[derive(Subcommand, Debug)]
pub enum BalancesSubcommand {
    /// List account balances
    List,
}

#[derive(Subcommand, Debug)]
pub enum TxSubcommand {
    /// Add transactions
    Add(AddArgs),
}

#[derive(Parser, Debug)]
pub struct AddArgs {
    #[arg(long, help = "From what account to send tokens")]
    pub from: String,

    #[arg(long, help = "To what account to send tokens")]
    pub to: String,

    #[arg(long, help = "How many tokens to send")]
    pub value: u64,

    #[arg(long, help = "Additional transaction data")]
    pub data: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Balances { command } => manage_balances(command)?,
        Commands::Tx { command } => manage_txs(command)?,
    }
    Ok(())
}
