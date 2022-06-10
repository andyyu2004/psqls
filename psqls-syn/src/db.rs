use std::sync::Arc;

use tree_sitter::Tree;

#[salsa::query_group(SyntaxDatabaseStorage)]
pub trait SyntaxDatabase {
    #[salsa::input]
    fn text(&self, url: String) -> Arc<str>;

    #[salsa::dependencies]
    fn parse(&self, url: String) -> Tree;
}

fn parse(db: &dyn SyntaxDatabase, url: String) -> Tree {
    crate::parse(&db.text(url))
}
