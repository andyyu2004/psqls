pub use self::db::{SyntaxDatabase, SyntaxDatabaseStorage};

mod convert;
mod db;
mod lower;
mod nodes;

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
