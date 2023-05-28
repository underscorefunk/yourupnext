pub enum Error {
    Command(String),
    Query(String),
    Generic(String),
}

pub fn cmd_err(error: &str) -> Error {
    Error::Command(error.into())
}

pub fn qry_err(error: &str) -> Error {
    Error::Query(error.into())
}

pub fn err(error: &str) -> Error {
    Error::Generic(error.into())
}
