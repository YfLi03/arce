use std::fmt::{self, Display};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub enum Reason {
    Database,
    Filesystem,
    Config,
    Internet,
    Internal
}

impl Display for Reason{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Reason::Database => "Sqlite or R2d2 Error",
            Reason::Filesystem => "File Notification Error",
            Reason::Config => "config.yaml Error",
            Reason::Internet => "SSH Error",
            Reason::Internal => "Tera or other Crates Error"
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Serialize)]
pub struct Error {
    pub reason: Reason,
    pub message: String
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} with message {}.", self.reason, self.message)
    }
}

impl std::error::Error for Error {

}

impl Error {
    pub fn new(reason: Reason, message: String) -> Self {
        Error {
            reason,
            message
        }
    }
}

impl From<rusqlite::Error> for Error{
    fn from(err: rusqlite::Error) -> Self {
        Error {
            reason: Reason::Database,
            message: err.to_string()
        }
    }
}


