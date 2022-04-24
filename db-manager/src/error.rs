use std::fmt::{Debug, Display, Error, Formatter};

pub enum ManagerError {
    InvalidConnectionString(String),
    UnsupportedDriver,
}

impl Display for ManagerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            ManagerError::InvalidConnectionString(connection_string) => write!(
                f,
                "The connection string supplied, `{}`, is invalid",
                connection_string
            ),
            ManagerError::UnsupportedDriver => write!(
                f,
                "The backend you're attempting to use is not an enabled feature."
            ),
        }
    }
}

impl Debug for ManagerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            ManagerError::InvalidConnectionString(connection_string) => write!(
                f,
                "ManagerError::InvalidConnectionString({})",
                connection_string
            ),
            ManagerError::UnsupportedDriver => write!(f, "ManagerError::UnsupportedDriver"),
        }
    }
}
