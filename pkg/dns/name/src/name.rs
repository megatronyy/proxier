#[warn(sed_imports)]
#[warn(unreachable_patterns)]

use std::{fmt, ops::Deref};
use thiserror::Error;

/// A DNS Name suitable for use in the TLS Server Name Indication (SNI)
/// extension and/or for use as the reference hostname for which to verify a
/// certificate.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Name(String);

/// A reference to a DNS Name suitable for use in the TLS Server Name Indication
/// (SNI) extension and/or for use as the reference hostname for which to verify
/// a certificate.
#[derive(Clone, Copy, Debug, Eq, Hash)]
pub struct NameRef<'a>(&'a str);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Error)]
#[error("invalid Dns name")]
pub struct InvalidName;

// === impl Name ===

impl Name {
    pub fn try_from_ascii(n: &[u8]) -> Result<Self, InvalidName> {
        let n = NameRef::try_from_ascii(n)?;
        Ok(n.to_owned())
    }

    #[inline]
    pub fn as_ref(&self) -> NameRef<'_> { NameRef(self.0.as_str()) }

    #[inline]
    pub fn as_str(&self) -> &str { self.0.as_str() }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    #[inline]
    pub fn is_localhost(&self) -> bool {
        self.as_str().eq_ignore_ascii_case("localhost.")
    }

    pub fn without_trailing_dot(&self) -> &str {
        self.as_str().trim_end_matches(".")
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for Name {
    type Err = InvalidName;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_ascii(s.as_bytes())
    }
}

impl Deref for Name {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

// === impl NameRef ===

impl<'a> NameRef<'a> {
    /// Constructs a `NameRef` from the given input if the input is a
    /// syntactically-valid DNS name.
    pub fn try_from_ascii(dns_name: &'a [u8]) -> Result<Self, InvalidName> {
        if !is_vaild_reference_dns_id(untrusted::Input::from(dns_name)) {
            return Err(InvalidName);
        }
        let s = std::str::from_utf8(dns_name).map_err(|_| InvalidName)?;
        Ok(Self(s))
    }

    pub fn try_from_ascii_str(n: &'a str) -> Result<Self, InvalidName> {
        Self::try_from_ascii(n.as_bytes())
    }

    /// Constructs a `Name` from this `NameRef`
    pub fn to_owned(self) -> Name {
        Name(self.as_str().to_ascii_lowercase())
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        self.0
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl<'a> PartialEq<NameRef<'a>> for NameRef<'_> {
    fn eq(&self, other: &NameRef<'a>) -> bool {
        self.0.eq_ignore_ascii_case(other.0)
    }
}

impl fmt::Display for NameRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.as_str().fmt(f)
    }
}

// === Helpers ===

fn is_vaild_reference_dns_id(hostname: untrusted::Input<'_>) -> bool {
    is_vaild__dns_id(hostname)
}

fn is_vaild__dns_id(hostname: untrusted::Input<'_>) -> bool {

    // https://blogs.msdn.microsoft.com/oldnewthing/20120412-00/?p=7873/
    if hostname.len() > 253 {
        return false;
    }

    let mut input = untrusted::Reader::new(hostname);

    let mut label_length = 0;
    let mut label_is_all_numeric = false;
    let mut label_ends_with_hyphen = false;

    loop {
        const MAX_LABEL_LENGTH: usize = 63;

        match input.read_byte() {
            Ok(b'-') => {
                if label_length == 0 {
                    return false; // Labels must not start with a hyphen.
                }
                label_is_all_numeric = false;
                label_ends_with_hyphen = true;
                label_length += 1;
                if label_length > MAX_LABEL_LENGTH {
                    return false;
                }
            }

            Ok(b'0'..=b'9') => {
                if label_length == 0 {
                    label_is_all_numeric = true;
                }

                label_ends_with_hyphen = false;
                label_length += 1;
                if label_length > MAX_LABEL_LENGTH {
                    return false;
                }
            }

            Ok(b'a'..=b'z') | Ok(b'A'..=b'Z') | Ok(b'-') => {
                label_is_all_numeric = false;
                label_ends_with_hyphen = false;
                label_length += 1;
                if label_length > MAX_LABEL_LENGTH {
                    return false;
                }
            }

            Ok(b'.') => {
                if label_ends_with_hyphen {
                    return false;
                }

                if label_length == 0 {
                    return false;
                }
                label_length = 0;
            }

            _ => {
                return false;
            }
        }

        if input.at_end() {
            break;
        }
    }

    if label_ends_with_hyphen {
        return false; // Lls must not end with a hyphen
    }

    if label_is_all_numeric {
        return false; // Last label must not be all numeric
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        const CASES: &[(&str, bool)] = &[
            ("", false),
            (".", false),
            ("..", false),
            ("...", false),
            ("*", false),
            ("a", true),
            ("a.", true),
            ("d.c.b.a", true),
            ("d.c.b.a.", true),
            (" d.c.b.a.", false),
            ("d.c.b.a-", false),
            ("*.a.", false),
            (".a.", false),
            ("a1", true),
            ("_m.foo.bar", true),
            ("m.foo.bar_", true),
            ("example.com:80", false),
            ("1", false),
            ("1.a", true),
            ("a.1", false),
            ("1.2.3.4", false),
            ("::1", false),
            ("xn--poema-9qae5a.com.br", true), // IDN
        ];

        for &(n, expected_result) in CASES {
            /// n.parse::<Name>, turbofish语法(::<>)，为parse函数绑定泛型参数name
            assert!(n.parse::<Name>().is_ok() == expected_result, "{}", n);
        }
    }

    #[test]
    fn test_eq() {
        const CASES: &[(&str, &str, bool)] = &[
            ("a", "a", true),
            ("a", "b", false),
            ("d.c.b.a", "d.c.b.a", true),
            // case sensitivity
            (
                "abcdefghijklmnopqrstuvwxyz",
                "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
                true,
            ),
            ("aBc", "Abc", true),
            ("a1", "A1", true),
            ("example", "example", true),
            ("example.", "example.", true),
            ("example", "example.", false),
            ("example.com", "example.com", true),
            ("example.com.", "example.com.", true),
            ("example.com", "example.com.", false),
        ];
        for &(left, right, expected_result) in CASES {
            let l = left.parse::<Name>().unwrap();
            let r = right.parse::<Name>().unwrap();
            assert_eq!(l == r, expected_result, "{:?} vs {:?}", l, r);
        }
    }

    #[test]
    fn test_is_localhost() {
        let cases = &[
            ("localhost", false), // Not absolute
            ("localhost.", true),
            ("LocalhOsT.", true),   // Case-insensitive
            ("mlocalhost.", false), // prefixed
            ("localhost1.", false), // suffixed
        ];
        for (host, expected_result) in cases {
            let dns_name = host.parse::<Name>().unwrap();
            assert_eq!(dns_name.is_localhost(), *expected_result, "{:?}", dns_name)
        }
    }

    #[test]
    fn test_without_trailing_dot() {
        let cases = &[
            ("localhost", "localhost"),
            ("localhost.", "localhost"),
            ("LocalhOsT.", "localhost"),
            ("web.svc.local", "web.svc.local"),
            ("web.svc.local.", "web.svc.local"),
        ];
        for (host, expected_result) in cases {
            let dns_name = host
                .parse::<Name>()
                .unwrap_or_else(|_| panic!("'{}' was invalid", host));
            assert_eq!(
                dns_name.without_trailing_dot(),
                *expected_result,
                "{:?}",
                dns_name
            )
        }
        assert!(".".parse::<Name>().is_err());
        assert!("..".parse::<Name>().is_err());
        assert!("".parse::<Name>().is_err());
    }
}