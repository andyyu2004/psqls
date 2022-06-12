use std::sync::Arc;

use psqls_syn::{SourceFile, SyntaxDatabase, SyntaxToken, TextRange, Visitor};

use crate::Snapshot;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HighlightRange {
    pub range: TextRange,
    pub hl: Highlight,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Highlight {
    Keyword,
}

impl Snapshot {
    pub fn highlight(&self, url: Arc<str>) -> Vec<HighlightRange> {
        let parsed = self.parse(url);
        let source = parsed.root();
        Highlighter::default().highlight(source)
    }
}

#[derive(Default)]
struct Highlighter {
    highlights: Vec<HighlightRange>,
}

impl Highlighter {
    pub fn highlight(mut self, source: SourceFile) -> Vec<HighlightRange> {
        self.visit_source_file(source);
        self.highlights
    }
}

impl Visitor for Highlighter {
    fn visit_kw(&mut self, kw: SyntaxToken) {
        self.highlights.push(HighlightRange {
            range: kw.text_range(),
            hl: Highlight::Keyword,
        })
    }
}

#[cfg(test)]
mod tests;
