/*
    后缀处理
*/
use std::{fmt, str::FromStr};
use std::fmt::{Formatter, Pointer};
use crate::Name;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Suffix {
    //The `.` stuffix.
    Root,
    Name(Name),
}

impl fmt::Display for Suffix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suffix::Root => write!(f, "."),
            Suffix::Name(n) => n.fmt(f),
        }
    }
}

impl From<Name> for Suffix {
    fn from(n: Name) -> Self {
        Suffix::Name(n)
    }
}

impl FromStr for Suffix {
    type Err = <Name as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "." {
            Ok(Suffix::Root)
        } else {
            Name::from_str(s).map(Suffix::Name)
        }
    }
}

impl Suffix {
    pub fn contains(&self, name: &Name) -> bool {
        match self {
            Suffix::Root => true,
            Suffix::Name(ref sfx) => {
                let name = name.without_trailing_dot();
                let sfx = sfx.without_trailing_dot();
                name.end_with(sfx) && {
                    name.len() == sfx.len() || {
                        let idx = name.len() - sfx.len();
                        let (hd, _) = name.split_at(idx);
                        hd.ends_with('.')
                    }
                }
            }
        }
    }
}