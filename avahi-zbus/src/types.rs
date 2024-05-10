use serde::{Deserialize, Serialize};
use zbus::zvariant::{NoneValue, Optional, Type};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Type, Deserialize, Serialize)]
#[repr(i32)]
pub enum Protocol {
    Unspec = -1,
    Inet = 0,
    Inet6 = 1,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Type, Deserialize, Serialize)]
#[zvariant(signature = "i")]
pub struct Interface(pub u16);

impl NoneValue for Interface {
    type NoneType = i32;

    fn null_value() -> Self::NoneType {
        -1
    }
}

impl From<i32> for Interface {
    fn from(value: i32) -> Self {
        Self(value.try_into().unwrap())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Type, Deserialize, Serialize)]
#[repr(i32)]
pub enum ServerState {
    Invalid,
    Registering,
    Running,
    Collision,
    Failure,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Type, Deserialize, Serialize)]
#[repr(i32)]
pub enum EntryGroupState {
    Uncommitted,
    Registering,
    Established,
    Collision,
    Failure,
}

#[derive(Debug, Clone, Copy)]
pub struct PublishFlags;

impl PublishFlags {
    pub const UNIQUE: i32 = 1;
    pub const NO_PROBE: i32 = 2;
    pub const NO_ANNOUNCE: i32 = 4;
    pub const ALLOW_MULTIPLE: i32 = 8;
    pub const NO_REVERSE: i32 = 16;
    pub const NO_COOKIE: i32 = 32;
    pub const UPDATE: i32 = 64;
    pub const USE_WIDE_AREA: i32 = 128;
    pub const USE_MULTICAST: i32 = 256;
}

#[derive(Debug, Clone, Copy)]
pub struct LookupFlags;

impl LookupFlags {
    pub const USE_WIDE_AREA: i32 = 1;
    pub const USE_MULTICAST: i32 = 2;
    pub const NO_TXT: i32 = 4;
    pub const NO_ADDRESS: i32 = 8;
}

#[derive(Debug, Clone, Copy)]
pub struct LookupResultFlags;

impl LookupResultFlags {
    pub const CACHED: i32 = 1;
    pub const WIDE_AREA: i32 = 2;
    pub const MULTICAST: i32 = 4;
    pub const LOCAL: i32 = 8;
    pub const OUR_OWN: i32 = 16;
    pub const STATIC: i32 = 32;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Type, Deserialize, Serialize)]
#[repr(i32)]
pub enum BrowserEvent {
    New,
    Remove,
    CacheExhausted,
    AllForNow,
    Failure,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Type, Deserialize, Serialize)]
#[repr(i32)]
pub enum ResolverEvent {
    Found,
    Failure,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Type, Deserialize, Serialize)]
#[repr(i32)]
pub enum DomainBrowserType {
    Browse,
    BrowseDefault,
    Register,
    RegisterDefault,
    BrowseLegacy,
    Max,
}

pub const DEFAULT_TTL_HOST_NAME: i32 = 120;
pub const DEFAULT_TTL: i32 = 75 * 60;

pub const DNS_CLASS_IN: u16 = 0x01;

#[derive(Debug, Clone, Copy)]
pub struct DnsType;

impl DnsType {
    pub const A: u16 = 0x01;
    pub const NS: u16 = 0x02;
    pub const CNAME: u16 = 0x05;
    pub const SOA: u16 = 0x06;
    pub const PTR: u16 = 0x0C;
    pub const HINFO: u16 = 0x0D;
    pub const MX: u16 = 0x0F;
    pub const TXT: u16 = 0x10;
    pub const AAAA: u16 = 0x1C;
    pub const SRV: u16 = 0x21;
}
