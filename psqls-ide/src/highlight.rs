use std::sync::Arc;

use psqls_syn::{Node, SyntaxDatabase, TextRange, Visitor};

use crate::Snapshot;

pub struct HighlightRange {
    pub range: TextRange,
    pub hl: Highlight,
}

pub enum Highlight {
    Keyword,
}

impl Snapshot {
    pub fn highlight(&self, url: Arc<str>) -> Vec<HighlightRange> {
        let parsed = self.parse(url);
        let source = parsed.root();
        vec![]
    }
}

struct HighlightVisitor {}

impl Visitor for HighlightVisitor {
    fn visit_false(&mut self, r#false: psqls_syn::False) {
        r#false.syntax().text_range();
    }
}
