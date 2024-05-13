use core::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ServiceError {
    #[error("Service type has more than 63 characters")]
    LongKind,
    #[error("Empty service type not allowed")]
    ShortKind,
    #[error("Service type contains invalid characters")]
    InvalidChar,
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, SerializeDisplay, DeserializeFromStr,
)]
pub struct ServiceKind(String);

impl ServiceKind {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for ServiceKind {
    type Err = ServiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ServiceError::ShortKind);
        } else if s.len() >= 64 {
            return Err(ServiceError::LongKind);
        }

        let mut buf = String::new();
        let mut chars = s.chars();

        let prefix = chars.next().unwrap();
        if prefix != '_' {
            buf.push(prefix);
        }

        for c in chars {
            if !is_valid(c) {
                return Err(ServiceError::InvalidChar);
            }

            buf.push(c)
        }

        Ok(Self(buf))
    }
}

fn is_valid(c: char) -> bool {
    (c.is_alphabetic() && c.is_lowercase()) || c == '-'
}

impl fmt::Display for ServiceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, SerializeDisplay, DeserializeFromStr,
)]
pub enum TransportProtocol {
    Tcp,
    Udp,
}

impl TransportProtocol {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Tcp => "tcp",
            Self::Udp => "udp",
        }
    }
}

impl FromStr for TransportProtocol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tcp" => Ok(Self::Tcp),
            "udp" => Ok(Self::Udp),
            _ => Err(s.to_owned()),
        }
    }
}

impl fmt::Display for TransportProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub kinds: Vec<ServiceKind>,
    pub protocol: TransportProtocol,
    pub port: u16,
}

impl Service {
    pub fn new(name: String, kind: ServiceKind, protocol: TransportProtocol, port: u16) -> Self {
        Self {
            name,
            kinds: vec![kind],
            protocol,
            port,
        }
    }

    pub fn with_sub_kinds(
        name: String,
        kinds: Vec<ServiceKind>,
        protocol: TransportProtocol,
        port: u16,
    ) -> Self {
        Self {
            name,
            kinds,
            protocol,
            port,
        }
    }
}

impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            name,
            kinds,
            protocol,
            port,
        } = self;

        write!(
            f,
            "{name} {{ Type: {}, Port: {port} }}",
            format_args!("_{}._{}", kinds[0], protocol)
        )
    }
}
