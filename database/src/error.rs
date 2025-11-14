#[derive(Debug, thiserror::Error)]
pub enum DBError{
  #[error("I/o error: {0}")]
  Io(std::io::Error),
}

impl From<std::io::Error> for DBError {
  fn from(err: std::io::Error) -> Self {
      DBError::Io(err)
  }
}