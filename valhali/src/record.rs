use avahi_zbus::Ttl;
use serde::{Deserialize, Serialize};

use crate::{name::NameBuf, rdata::RecordData};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Record<D: RecordData> {
    pub name: NameBuf,
    pub ttl: Ttl,
    pub data: D,
}

impl<D: RecordData> Record<D> {
    pub fn new(name: NameBuf, ttl: Ttl, data: D) -> Self {
        Self { name, ttl, data }
    }
}
