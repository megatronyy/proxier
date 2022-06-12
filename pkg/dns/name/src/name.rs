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
#[derive(Clone, Copy, Debug, Eq, )]
pub struct NameRef<'a>(&'a str);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Error)]
#[error("invalid Dns name")]
pub struct InvalidName;

// === impl Name ===

impl Name {
    pub fn try_from_ascii(n: &[u8]) -> Result<Self, InvalidName> {
        //let n =
        todo!()
    }
}

impl<'a> NameRef<'a> {
    pub fn try_from_ascii(dns_name: &'a [u8]) -> Result<Self, InvalidName> {
        todo!()
    }
}

// === Helpers ===

fn is_vaild_reference_dns_id(hostname: untrusted::Input<'_>) -> bool {
    todo!()
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

    fn test_parse() {
        todo!()
    }
}