use lib0::error::Error;

pub fn to_sqlite_error(x: Error) -> sqlite_loadable::Error {
    sqlite_loadable::Error::new_message(x.to_string().as_str())
}