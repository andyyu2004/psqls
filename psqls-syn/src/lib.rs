#![cfg_attr(test, feature(box_patterns))]

pub use self::db::{SyntaxDatabase, SyntaxDatabaseStorage};
pub use self::generated::*;
pub use self::node::Node;
pub use ropey::Rope;
pub use rowan::{TextRange, TextSize};

mod convert;
mod db;
mod generated;
mod lower;
mod node;
mod validation;
mod visit;

use tree_sitter::{Language, Parser, Tree};

extern "C" {
    fn tree_sitter_sql() -> Language;
}

pub fn sql() -> Language {
    unsafe { tree_sitter_sql() }
}

pub fn parse(text: &str) -> Tree {
    let mut parser = Parser::new();
    parser.set_language(sql()).unwrap();
    parser.parse(text, None).unwrap()
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod gen;
