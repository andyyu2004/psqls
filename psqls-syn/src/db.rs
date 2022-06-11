use std::marker::PhantomData;
use std::sync::Arc;

use rowan::Language;
use tree_sitter::Tree;

use crate::convert;
use crate::node::{GreenNode, Node, Sql, SyntaxNode};
use crate::nodes::{SourceFile, SyntaxKind};

#[salsa::query_group(SyntaxDatabaseStorage)]
pub trait SyntaxDatabase {
    #[salsa::input]
    fn text(&self, url: String) -> Arc<str>;

    #[salsa::dependencies]
    fn parse_raw(&self, url: String) -> Tree;

    fn parse(&self, url: String) -> Parse<SourceFile>;
}

fn parse_raw(db: &dyn SyntaxDatabase, url: String) -> Tree {
    crate::parse(&db.text(url))
}

#[derive(Debug, PartialEq, Eq)]
pub struct Parse<T> {
    green: GreenNode,
    _type: PhantomData<fn() -> T>,
}

impl<T: Node> Parse<T> {
    pub(crate) fn new(green: GreenNode) -> Self {
        assert!(T::can_cast(Sql::kind_from_raw(green.kind())));
        Self {
            green,
            _type: PhantomData,
        }
    }

    pub fn root(&self) -> T {
        T::cast(SyntaxNode::new_root(self.green.clone())).unwrap()
    }
}

impl<T> Clone for Parse<T> {
    fn clone(&self) -> Parse<T> {
        Parse {
            green: self.green.clone(),
            _type: PhantomData,
        }
    }
}

fn parse(db: &dyn SyntaxDatabase, url: String) -> Parse<SourceFile> {
    let tree = db.parse_raw(url.clone());
    let green = convert::ts_to_rowan(db.text(url), tree);
    assert_eq!(green.kind(), Sql::kind_to_raw(SyntaxKind::SourceFile));
    Parse::new(green)
}
