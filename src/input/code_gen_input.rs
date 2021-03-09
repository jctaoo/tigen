use std::fmt;
use std::io::Error;

pub trait CodeGenInput {
    fn absolute_path() -> Result<&str, Error>;
}

