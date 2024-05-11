use core::fmt;
use std::{ops::Deref, str::FromStr};

use avahi_zbus::DnsType;

use crate::name::{Name, NameBuf, NameError};

pub trait RecordData {
    const KIND: DnsType;

    fn as_rdata(&self) -> &[u8];
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cname(NameBuf);

impl Cname {
    pub fn as_name(&self) -> &Name {
        self.0.as_ref()
    }
}

impl From<NameBuf> for Cname {
    fn from(value: NameBuf) -> Self {
        Self(value)
    }
}

impl FromStr for Cname {
    type Err = NameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buf = NameBuf::from_str(s)?;

        Ok(Self(buf))
    }
}

impl fmt::Display for Cname {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl RecordData for Cname {
    const KIND: DnsType = DnsType::CNAME;

    fn as_rdata(&self) -> &[u8] {
        self.0.as_slice()
    }
}
