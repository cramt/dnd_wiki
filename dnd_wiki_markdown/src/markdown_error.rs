use thiserror::Error;
use shoulda::Shoulda;

#[derive(Debug, Error, Shoulda)]
pub enum MarkdownError {
    #[error("unknown reference {path:?} with name {name:?}")]
    UnknownReference{
        name: String,
        path: Vec<String>,
    }
}