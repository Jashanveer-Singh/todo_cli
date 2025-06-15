use std::io;

pub enum MyError {
    Io(io::Error),
    Serde(serde_json::Error),
    Parse(std::num::ParseIntError),
}

impl From<io::Error> for MyError {
    fn from(err: io::Error) -> MyError {
        MyError::Io(err)
    }
}

impl From<serde_json::Error> for MyError {
    fn from(err: serde_json::Error) -> MyError {
        MyError::Serde(err)
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(err: std::num::ParseIntError) -> MyError {
        MyError::Parse(err)
    }
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::Serde(error)=> write!(f, "{}", error),
            MyError::Parse(error)=> write!(f, "{}", error),
            MyError::Io(error)=> write!(f, "{}", error),
        }
    }
}
