use std::marker::PhantomData;
use std::sync::Arc;

use ropey::Rope;
use rowan::Language;
use tree_sitter::Tree;

use crate::generated::{SourceFile, SyntaxKind};
use crate::node::{GreenNode, Node, Sql, SyntaxNode};
use crate::ts_to_rowan;

#[salsa::query_group(SyntaxDatabaseStorage)]
pub trait SyntaxDatabase {
    #[salsa::input]
    fn text(&self, url: Arc<str>) -> Arc<str>;

    fn rope(&self, url: Arc<str>) -> Rope;

    #[salsa::dependencies]
    fn parse_raw(&self, url: Arc<str>) -> Tree;

    fn parse(&self, url: Arc<str>) -> Parse<SourceFile>;
}

fn rope(db: &dyn SyntaxDatabase, url: Arc<str>) -> Rope {
    Rope::from_str(db.text(url).as_ref())
}

fn parse_raw(db: &dyn SyntaxDatabase, url: Arc<str>) -> Tree {
    crate::parse(&db.text(url))
}

fn parse(db: &dyn SyntaxDatabase, url: Arc<str>) -> Parse<SourceFile> {
    let tree = db.parse_raw(url.clone());
    let green = ts_to_rowan::ts_to_rowan(db.text(url), tree);
    assert_eq!(Sql::kind_from_raw(green.kind()), SyntaxKind::SourceFile);
    Parse::new(green)
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
