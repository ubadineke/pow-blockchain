// Define a custom error type
#[derive(Debug)]
pub enum GenesisError {
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl From<std::io::Error> for GenesisError {
    fn from(err: std::io::Error) -> Self {
        GenesisError::Io(err)
    }
}

impl From<serde_json::Error> for GenesisError {
    fn from(err: serde_json::Error) -> Self {
        GenesisError::Json(err)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StateError {
    #[error("I/O error: {0}")]
    Io(std::io::Error),

    #[error("JSON error: {0}")]
    Json(serde_json::Error),

    #[error(
        "Insufficient balance in account {account}: requested {requested}, available {available}"
    )]
    InsufficientBalance {
        account: String,
        requested: u64,
        available: u64,
    },

    #[error("Hex decode error: {0}")]
    HexDecode(hex::FromHexError),

    #[error("expected 32 bytes but got {0}")]
    InvalidLength(usize),
}

impl From<std::io::Error> for StateError {
    fn from(err: std::io::Error) -> Self {
        StateError::Io(err)
    }
}

impl From<serde_json::Error> for StateError {
    fn from(err: serde_json::Error) -> Self {
        StateError::Json(err)
    }
}

impl From<hex::FromHexError> for StateError {
    fn from(err: hex::FromHexError) -> Self {
        StateError::HexDecode(err)
    }
}

//WOULD TRY THIS OUT AFTER SEEEING WHAT THE BOX ERROR CCAN OFFER
// #[derive(Debug, thiserror::Error)]
// pub enum CliError {
//     #[error(transparent)]
//     State(#[from] StateError),

//     #[error(transparent)]
//     Genesis(#[from] GenesisError),
// }
