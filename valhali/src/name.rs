use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{
    fmt,
    ops::Deref,
    str::{from_utf8_unchecked, FromStr},
};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum NameError {
    EmptyLabel,
    LongName,
    LongLabel,
}

impl fmt::Display for NameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Label([u8]);

impl Label {
    fn split_from(slice: &[u8]) -> (&Self, &[u8]) {
        let len = slice[0];
        let (left, right) = slice[1..].split_at(len as usize);

        let label = unsafe { &*(left as *const [u8] as *const Label) };

        (label, right)
    }

    pub fn as_str(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.0) }
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NameIter<'a>(&'a [u8]);

impl<'a> Iterator for NameIter<'a> {
    type Item = &'a Label;

    fn next(&mut self) -> Option<Self::Item> {
        if let &[0u8] = self.0 {
            None
        } else {
            let (label, right) = Label::split_from(&self.0);
            self.0 = right;
            Some(label)
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Name([u8]);

impl Name {
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn iter(&self) -> NameIter {
        self.into_iter()
    }

    pub fn root(&self) -> &Label {
        self.iter().next().unwrap()
    }

    pub fn is_root(&self) -> bool {
        self.iter().count() == 1
    }

    pub fn starts_with(&self, base: impl AsRef<Name>) -> bool {
        self.0.starts_with(&base.as_ref().0)
    }

    pub fn ends_with(&self, base: impl AsRef<Name>) -> bool {
        self.0.ends_with(&base.as_ref().0)
    }
}

impl<'a> IntoIterator for &'a Name {
    type IntoIter = NameIter<'a>;
    type Item = &'a Label;

    fn into_iter(self) -> Self::IntoIter {
        NameIter(&self.0)
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.iter();
        let first = iter.next().unwrap();
        write!(f, "{first}")?;

        for label in iter {
            write!(f, ".{label}")?;
        }

        Ok(())
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, SerializeDisplay, DeserializeFromStr,
)]
pub struct NameBuf(Vec<u8>);

impl NameBuf {
    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }
}

impl FromStr for NameBuf {
    type Err = NameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len() + 1;
        if len >= 255 {
            return Err(NameError::LongName);
        }

        let mut buf = Vec::with_capacity(len);
        for label in s.split('.') {
            if label.is_empty() {
                return Err(NameError::EmptyLabel);
            }
            if label.len() >= 64 {
                return Err(NameError::LongLabel);
            }

            buf.push(label.len() as u8);
            buf.extend(label.as_bytes());
        }
        buf.push(0u8);
        Ok(Self(buf))
    }
}

impl AsRef<Name> for NameBuf {
    fn as_ref(&self) -> &Name {
        unsafe { &*(self.0.as_slice() as *const [u8] as *const Name) }
    }
}

impl Deref for NameBuf {
    type Target = Name;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl fmt::Display for NameBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::NameBuf;

    #[test]
    fn name() {
        let domain = "a.local";
        let name = NameBuf::from_str(domain).unwrap();

        assert_eq!(
            name.as_slice(),
            &[1, b'a', 5, b'l', b'o', b'c', b'a', b'l', 0]
        )
    }
}
