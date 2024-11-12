// Copyright 2016 mime-multipart Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::borrow::Cow;
use std::error::Error as StdError;
use std::fmt::{self, Display};
use std::io;
use std::string::FromUtf8Error;

use httparse;
use hyper;

/// An error type for the `mime-multipart` crate.
pub enum Error {
    /// The Hyper request did not have a Content-Type header.
    NoRequestContentType,
    /// The Hyper request Content-Type top-level Mime was not `Multipart`.
    NotMultipart,
    /// The Content-Type header failed to specify boundary token.
    BoundaryNotSpecified,
    /// A multipart section contained only partial headers.
    PartialHeaders,
    EofInMainHeaders,
    EofBeforeFirstBoundary,
    NoCrLfAfterBoundary,
    EofInPartHeaders,
    EofInFile,
    EofInPart,
    /// An HTTP parsing error from a multipart section.
    Httparse(httparse::Error),
    /// An I/O error.
    Io(io::Error),
    /// An error was returned from Hyper.
    Hyper(hyper::Error),
    /// An error occurred during UTF-8 processing.
    Utf8(FromUtf8Error),
    /// An error occurred during character decoding
    Decoding(Cow<'static, str>),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<httparse::Error> for Error {
    fn from(err: httparse::Error) -> Error {
        Error::Httparse(err)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Hyper(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        Error::Utf8(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NoRequestContentType => 
                write!(f, "No request Content-Type"),
            Error::NotMultipart => 
                write!(f, "Not a multipart"),
            Error::BoundaryNotSpecified => 
                write!(f, "Boundary not specified"),
            Error::PartialHeaders => 
                write!(f, "Partial headers"),
            Error::EofInMainHeaders => 
                write!(f, "EOF in Main headers"),
            Error::EofBeforeFirstBoundary => 
                write!(f, "EOF Before First boundary"),
            Error::NoCrLfAfterBoundary => 
                write!(f, "No CR-LF After boundary"),
            Error::EofInPartHeaders => 
                write!(f, "EOF in Partial headers"),
            Error::EofInFile => 
                write!(f, "EOF in File"),
            Error::EofInPart => 
                write!(f, "EOF in Part"),
            Error::Httparse(error) =>
                write!(f, "Httparse: {error}"),
            Error::Io(ref e) =>
                format!("Io: {}", e).fmt(f),
            Error::Hyper(ref e) =>
                format!("Hyper: {}", e).fmt(f),
            Error::Utf8(ref e) =>
                format!("Utf8: {}", e).fmt(f),
            Error::Decoding(ref e) =>
                format!("Decoding: {}", e).fmt(f),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)?;
        if self.source().is_some() {
            write!(f, ": {:?}", self.source().unwrap())?; // recurse
        }
        Ok(())
    }
}

impl StdError for Error {
    fn description(&self) -> &str{
        match *self {
            Error::NoRequestContentType => "The Hyper request did not have a Content-Type header.",
            Error::NotMultipart =>
                "The Hyper request Content-Type top-level Mime was not multipart.",
            Error::BoundaryNotSpecified =>
                "The Content-Type header failed to specify a boundary token.",
            Error::PartialHeaders =>
                "A multipart section contained only partial headers.",
            Error::EofInMainHeaders =>
                "The request headers ended pre-maturely.",
            Error::EofBeforeFirstBoundary =>
                "The request body ended prior to reaching the expected starting boundary.",
            Error::NoCrLfAfterBoundary =>
                "Missing CRLF after boundary.",
            Error::EofInPartHeaders =>
                "The request body ended prematurely while parsing headers of a multipart part.",
            Error::EofInFile =>
                "The request body ended prematurely while streaming a file part.",
            Error::EofInPart =>
                "The request body ended prematurely while reading a multipart part.",
            Error::Httparse(_) =>
                "A parse error occurred while parsing the headers of a multipart section.",
            Error::Io(_) => "An I/O error occurred.",
            Error::Hyper(_) => "A Hyper error occurred.",
            Error::Utf8(_) => "A UTF-8 error occurred.",
            Error::Decoding(_) => "A decoding error occurred.",
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::*;

    /// ensure an Error::Decoding value can be formatted with std::fmt::Display without panic
    #[test]
    fn test_decoding_error() {
        let test_error = Error::Decoding(std::borrow::Cow::Borrowed("some message"));
        test_error.to_string();
    }

    /// ensure an Error::NotMultipart value can be formatted with std::fmt::Display without panic
    #[test]
    fn test_not_multipart() {
        let test_error = Error::NotMultipart;
        test_error.to_string(); // calling this should not panic
    }
}