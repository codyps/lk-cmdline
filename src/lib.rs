//! Parse the linux kernel command line (`/proc/cmdline`)

use std::borrow::Cow;

/// A command line reference
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmdLine<'a> {
    data: &'a [u8],
}

impl<'a> CmdLine<'a> {
    /// Create the command line from a string
    pub fn from_str(data: &'a str) -> Self {
        Self {
            data: data.as_bytes(),
        }
    }

    pub fn from_bytes(data: &'a [u8]) -> Self {
        Self { data }
    }

    /// Iterate over elements of the command line
    pub fn iter(&self) -> CmdLineIter<'a> {
        CmdLineIter { remain: self.data }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmdLineIter<'a> {
    remain: &'a [u8],
}

/// An argument on the cmdline
///
// NOTE: because there is no escaping and only use of double quotes, when determining actual
// values, it's sufficient to just skip over double quotes. No states needed
pub struct Arg<'a> {
    data: &'a [u8],
    eq_offs: usize,
}

impl<'a> Arg<'a> {
    pub fn with_eq_offs(data: &'a [u8], eq_offs: usize) -> Self {
        assert!(data.len() == eq_offs || data[eq_offs] == b'=');
        // other things we could validate:
        //  - that bytes do represent a single arg (check quoting)

        Self { data, eq_offs }
    }

    pub fn from_bytes(data: &'a [u8]) -> Self {
        let eq_offs = memchr::memchr(b'=', data).unwrap_or(data.len());
        Self::with_eq_offs(data, eq_offs)
    }

    /// Check if a parameter matches, using the lkcmdline matching rules
    ///
    /// Specifically, `-` (dash) and `_` (underscore) are considered interchangeable
    pub fn param_matches(&self, param: &[u8]) -> bool {
        todo!()
    }

    pub fn param(&self) -> Cow<'a, [u8]> {
        todo!()
    }

    pub fn value(&self) -> Option<Cow<'a, [u8]>> {
        todo!()
    }
}

impl<'a> Iterator for CmdLineIter<'a> {
    // Note: allocations here are because quotes may be in the middle of values. These strings
    // parse out the quotes.
    type Item = Arg<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut rem = self.remain;

        // skip spaces
        while !rem.is_empty() {
            let c = rem[0];
            if c.is_ascii_whitespace() {
                rem = &rem[1..];
            } else {
                break;
            }
        }

        // if we're empty, nothing was found. terminate
        if rem.is_empty() {
            self.remain = b"";
            return None;
        }

        // otherwise, we're probably at the start of a parameter.
        // parameters can have quotes, so we respect that
        let (arg_start, mut in_quote) = if rem[0] == b'"' {
            rem = &rem[1..];
            (rem, true)
        } else {
            (rem, false)
        };

        let mut p = 0;
        let mut eq_offs = None;
        while !rem.is_empty() {
            let c = rem[0];
            if eq_offs.is_none() && c == b'=' {
                eq_offs = Some(p)
            }
            p += 1;
            if c.is_ascii_whitespace() && !in_quote {
                // done, no value
                self.remain = &rem[1..];
                return Some(Arg::with_eq_offs(&arg_start[..p], eq_offs.unwrap_or(p)));
            } else if c == b'"' {
                in_quote = !in_quote;
            }
        }

        todo!()
    }
}
