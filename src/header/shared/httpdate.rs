use std::fmt::{self, Display};
use std::str::FromStr;

// use time;
use chrono::{DateTime, Utc};

/// A `time::Time` with HTTP formatting and parsing
///
//   Prior to 1995, there were three different formats commonly used by
//   servers to communicate timestamps.  For compatibility with old
//   implementations, all three are defined here.  The preferred format is
//   a fixed-length and single-zone subset of the date and time
//   specification used by the Internet Message Format [RFC5322].
//
//     HTTP-date    = IMF-fixdate / obs-date
//
//   An example of the preferred format is
//
//     Sun, 06 Nov 1994 08:49:37 GMT    ; IMF-fixdate
//
//   Examples of the two obsolete formats are
//
//     Sunday, 06-Nov-94 08:49:37 GMT   ; obsolete RFC 850 format
//     Sun Nov  6 08:49:37 1994         ; ANSI C's asctime() format
//
//   A recipient that parses a timestamp value in an HTTP header field
//   MUST accept all three HTTP-date formats.  When a sender generates a
//   header field that contains one or more timestamps defined as
//   HTTP-date, the sender MUST generate those timestamps in the
//   IMF-fixdate format.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
// pub struct HttpDate(pub time::Tm);
pub struct HttpDate(pub DateTime<Utc>);

impl FromStr for HttpDate {
    type Err = ::Error;
    fn from_str(s: &str) -> ::Result<HttpDate> {
        // match time::strptime(s, "%a, %d %b %Y %T %Z").or_else(|_| {
        //     time::strptime(s, "%A, %d-%b-%y %T %Z")
        //     }).or_else(|_| {
        //         time::strptime(s, "%c")
        //         }) {
        //             Ok(t) => Ok(HttpDate(t)),
        //             Err(_) => Err(::Error::Header),
        //             }
        match DateTime::parse_from_rfc2822(s)
            .or_else(|_| DateTime::parse_from_rfc3339(s))
            .or_else(|_| DateTime::parse_from_str(s, "%a, %d %b %Y %T %Z"))
            .or_else(|_| DateTime::parse_from_str(s, "%A, %d-%b-%y %T %Z"))
        {
            Ok(t) => {
                let time = DateTime::<Utc>::from_utc(t.naive_utc(), Utc);
                Ok(HttpDate(time))
            }
            Err(_) => Err(::Error::Header),
        }
    }
}

impl Display for HttpDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // fmt::Display::fmt(&self.0.to_utc().rfc822(), f)
        fmt::Display::fmt(&self.0.to_rfc2822(), f)
    }
}
