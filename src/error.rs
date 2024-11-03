use thiserror::Error;
#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum InterpeterError {
    #[error("Invalid syntax")]
    InvalidSyntax,
}
