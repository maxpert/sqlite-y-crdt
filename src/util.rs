use lib0::error::Error;
use sqlite_loadable::{Error as SQLiteError};

pub fn to_sqlite_error(x: Error) -> SQLiteError {
    SQLiteError::new_message(x.to_string().as_str())
}