use log::warn;
use serde::Serialize;
use std::fmt::{self, Display};

#[derive(Clone, Debug, Serialize)]
pub enum Reason {
    Database,
    ArticleRender,
    Filesystem,
    PictureProcess,
    // Config,       Config errors are unwrapped directly
    // Internet,     Internet errors are addressed directly
    Internal,
}

impl Display for Reason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Reason::Database => "Sqlite or R2d2 Error",
            Reason::Filesystem => "File Notification Error",
            // Reason::Config => "config.yaml Error",
            // Reason::Internet => "SSH Error",
            Reason::Internal => "Tera or other Crates Error",
            Reason::PictureProcess => "Error Processing the Image",
            Reason::ArticleRender => "Error Rendering Markdown to HTML",
        };
        write!(f, "{}", s)
    }
}

/// Self defined Error type, so that errors are passed elegantly
#[derive(Debug, Serialize)]
pub struct Error {
    pub reason: Reason,
    pub message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} with message {}.", self.reason, self.message)
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn new(reason: Reason, message: String) -> Self {
        Error { reason, message }
    }
}

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Self {
        warn!("{:?}", err);
        Error {
            reason: Reason::Database,
            message: err.to_string(),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        warn!("{:?}", err);
        Error {
            reason: Reason::Internal,
            message: err.to_string(),
        }
    }
}

impl From<notify::Error> for Error {
    fn from(err: notify::Error) -> Self {
        warn!("{:?}", err);
        Error {
            reason: Reason::Filesystem,
            message: err.to_string(),
        }
    }
}

impl From<image::ImageError> for Error {
    fn from(err: image::ImageError) -> Self {
        warn!("{:?}", err);
        Error {
            reason: Reason::PictureProcess,
            message: err.to_string(),
        }
    }
}

impl From<r2d2::Error> for Error {
    fn from(err: r2d2::Error) -> Self {
        warn!("{:?}", err);
        Error {
            reason: Reason::Database,
            message: err.to_string(),
        }
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Self {
        warn!("{:?}", err);
        Error {
            reason: Reason::Internal,
            message: err.to_string(),
        }
    }
}

impl From<sitemap::Error> for Error {
    fn from(err: sitemap::Error) -> Self {
        warn!("{:?}", err);
        Error {
            reason: Reason::Filesystem,
            message: err.to_string(),
        }
    }
}
