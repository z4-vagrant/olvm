pub mod structs;

use std;
use std::error::Error as StdError;
use std::fmt;
use std::io;

use serde_json;
use mongodb;

use config;

/*
 * Global context struct
 */
pub struct Context {
    pub conf: config::Config,
    pub db: mongodb::db::Database
}

/*
 * Error type
 */
pub struct Error {
    message: String
}

impl Error {
    pub fn new<S: Into<String>>(message: S) -> Error {
        Error {
            message: message.into()
        }
    }

    pub fn description_json(&self) -> String {
        format!("{{\"error\": \"{}\"}}", self.description())
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        self.message.as_str()
    }

    fn cause(&self) -> Option<&std::error::Error> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl std::convert::From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::new(format!("I/O error: {}", e.description()))
    }
}

impl std::convert::From<mongodb::Error> for Error {
    fn from(e: mongodb::Error) -> Error {
        Error::new(format!("Database error: {}", e.description()))
    }
}

impl std::convert::From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::new(format!("JSON error: {}", e.description()))
    }
}

/*
 * Define a Result type using our Error type
 */
pub type Result<T> = std::result::Result<T, Error>;
