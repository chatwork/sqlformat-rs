//! This crate is a port of https://github.com/kufii/sql-formatter-plus
//! written in Rust. It is intended to be usable as a pure-Rust library
//! for formatting SQL queries.

#![type_length_limit = "99999999"]
#![forbid(unsafe_code)]
// Maintains semver compatibility for older Rust versions
#![allow(clippy::manual_strip)]

mod formatter;
mod tokenizer;

pub fn format(query: &str, params: &QueryParams) -> String {
    let named_placeholders = matches!(params, QueryParams::Named(_));

    let tokens = tokenizer::tokenize(query, named_placeholders);
    formatter::format(&tokens)
}

/// Options for controlling how the library formats SQL
#[derive(Debug, Clone, Copy)]
pub struct FormatOptions {
    /// Controls the type and length of indentation to use
    ///
    /// Default: 2 spaces
    pub indent: Indent,
    /// When set, changes reserved keywords to ALL CAPS
    ///
    /// Default: false
    pub uppercase: bool,
    /// Controls the number of line breaks after a query
    ///
    /// Default: 1
    pub lines_between_queries: u8,
}

impl Default for FormatOptions {
    fn default() -> Self {
        FormatOptions {
            indent: Indent::Spaces(2),
            uppercase: false,
            lines_between_queries: 1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Indent {
    Spaces(u8),
    Tabs,
}

#[derive(Debug, Clone)]
pub enum QueryParams {
    Named(Vec<(String, String)>),
    Indexed(Vec<String>),
    None,
}

impl Default for QueryParams {
    fn default() -> Self {
        QueryParams::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_1() {
        let input = r"
            SELECT
                count(*),
                Column1
            FROM
                Table1 AS t1
            LEFT JOIN
                Table2 AS t2
            ON
                t2.hoge = t1.fuga
            WHERE
                id = 'hogehoge'
        ";
        let expected =
            indoc!("SELECT count(*), Column1 FROM Table1 AS t1 LEFT JOIN Table2 AS t2 ON t2.hoge = t1.fuga WHERE id = 'hogehoge'");

        assert_eq!(format(input, &QueryParams::None), expected);
    }
}
