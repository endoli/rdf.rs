// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Parse RDF

#![allow(missing_docs)]

named!(pub iriref <&str, &str>,
    do_parse!(
        tag_s!("<") >>
        iri: is_not_s!(">") >>
        tag_s!(">") >>
        (iri)));

named!(pub blanknodelabel <&str, &str>,
    do_parse!(
        tag_s!("_:") >>
        bnode: is_not_s!(" ") >>
        (bnode)));

named!(pub datatype <&str, &str>,
    do_parse!(
        tag_s!("^^") >>
        datatype_url: call!(iriref) >>
        (datatype_url)));

named!(pub langtag <&str, &str>,
    do_parse!(
        tag_s!("@") >>
        lang: is_not_s!(" ") >>
        (lang)));

named!(pub string_literal_quote <&str, &str>,
    do_parse!(
        tag_s!("\"") >>
        string: is_not_s!("\"") >>
        tag_s!("\"") >>
        (string)));

named!(pub literal <&str, &str>,
    do_parse!(
        lit: call!(string_literal_quote) >>
        trailer: opt!(alt!(langtag | datatype)) >>
        (lit)));

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult;

    #[test]
    fn can_parse_iriref() {
        match iriref("<foo>") {
            IResult::Done(_, m) => assert_eq!(m, "foo"),
            _ => panic!("Expected successful IRIREF parse."),
        }
    }

    #[test]
    fn can_parse_datatype() {
        match datatype("^^<foo>") {
            IResult::Done(_, m) => assert_eq!(m, "foo"),
            _ => panic!("Expected successful IRIREF parse."),
        }
    }

    #[test]
    fn can_parse_blanknodelabel() {
        match blanknodelabel("_:bar") {
            IResult::Done(_, m) => assert_eq!(m, "bar"),
            _ => panic!("Expected successful blank node label parse."),
        }
    }
}
