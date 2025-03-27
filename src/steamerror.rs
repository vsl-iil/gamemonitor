use core::fmt;
use std::{error, num::ParseIntError};

pub type Result<T> = std::result::Result<T, GeneralError>;
pub type SteamResult<T> = std::result::Result<T, SteamError>;
// pub type ArgumentResult<T> = std::result::Result<T, ArgumentError>;

#[derive(Debug)]
pub enum SteamError {
    RequestError(reqwest::Error),
    SerdeError(serde_json::Error),
}

impl fmt::Display for SteamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SteamError::RequestError(..) => write!(f, "Request to Steam Web API failed"),
            SteamError::SerdeError(..)   => write!(f, "Parsing Steam Web API answer failed"),
        }
    }
}


impl error::Error for SteamError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            SteamError::RequestError(ref re) => Some(re),
            SteamError::SerdeError(ref se)   => Some(se),
        }
    }
}


impl From<reqwest::Error> for SteamError {
    fn from(value: reqwest::Error) -> Self {
        SteamError::RequestError(value)
    }
}

impl From<serde_json::Error> for SteamError {
    fn from(value: serde_json::Error) -> Self {
        SteamError::SerdeError(value)
    }
}


#[derive(Debug)]
pub enum ArgumentError {
    ParseError(ParseIntError),
    UnitError,
    DelayError
}

impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ArgumentError::ParseError(..) => write!(f, "Parsing user input failed"),
            ArgumentError::UnitError      => write!(f, "Wrong delay unit specified"),
            ArgumentError::DelayError     => write!(f, "Specified delay is too short"),
        }
    }
}

impl error::Error for ArgumentError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ArgumentError::ParseError(ref pe) => Some(pe),
            _ => None,
        }
    }
}

impl From<ParseIntError> for ArgumentError {
    fn from(value: ParseIntError) -> Self {
        ArgumentError::ParseError(value)
    }
}


#[derive(Debug)]
pub enum GeneralError {
    Steam(SteamError),
    Argument(ArgumentError),
    Telegram(frankenstein::Error),
}

impl fmt::Display for GeneralError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            GeneralError::Steam(..)    => write!(f, "Steam error"),
            GeneralError::Argument(..) => write!(f, "Argument error"),
            GeneralError::Telegram(..) => write!(f, "Telegram error"),
        }
    }
}

impl error::Error for GeneralError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            GeneralError::Steam(ref se) => Some(se),
            GeneralError::Argument(ref ae) => Some(ae),
            GeneralError::Telegram(ref te) => Some(te),
        }
    }
}

impl From<SteamError> for GeneralError {
    fn from(value: SteamError) -> Self {
        GeneralError::Steam(value)
    }
}

impl From<ArgumentError> for GeneralError {
    fn from(value: ArgumentError) -> Self {
        GeneralError::Argument(value)
    }
}

impl From<frankenstein::Error> for GeneralError {
    fn from(value: frankenstein::Error) -> Self {
        GeneralError::Telegram(value)
    }
}
