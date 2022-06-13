use std::sync::Arc;

use rowan::{GreenNode, GreenNodeBuilder, Language};
use tree_sitter::{Node, Tree};

use crate::generated::SyntaxKind;
use crate::node::Sql;

// the reason for using rowan at all is due to treesitter nodes having an inconvenient
// lifetime which doesn't work well with salsa
// You could also argue why we even bother with salsa for this..
pub(crate) fn ts_to_rowan(text: Arc<str>, tree: Tree) -> GreenNode {
    Builder::new(text).build(tree)
}

struct Builder {
    text: Arc<str>,
    builder: GreenNodeBuilder<'static>,
    prev_end: usize,
}

impl Builder {
    fn new(text: Arc<str>) -> Self {
        Self {
            text,
            builder: Default::default(),
            prev_end: 0,
        }
    }

    fn build(mut self, tree: Tree) -> GreenNode {
        let root = tree.root_node();
        self.visit_root(root);
        self.builder.finish()
    }

    fn visit_root(&mut self, root: Node<'_>) {
        self.visit_node(root);
    }

    fn pad_whitespace(&mut self, start: usize) {
        if self.prev_end < start {
            let text = &self.text[self.prev_end..start];
            assert!(
                text.chars().all(char::is_whitespace),
                "non-whitespace padding `{text}`",
            );
            self.builder
                .token(Sql::kind_to_raw(SyntaxKind::Whitespace), text);
        }
    }

    fn token(&mut self, node: Node<'_>) {
        let kind = SyntaxKind::try_from(node.kind()).unwrap_or(SyntaxKind::Token);
        self.pad_whitespace(node.start_byte());
        self.prev_end = node.end_byte();
        self.builder.token(
            Sql::kind_to_raw(kind),
            node.utf8_text(self.text.as_bytes()).unwrap(),
        );
    }

    fn visit_node(&mut self, node: Node<'_>) {
        let kind = Sql::kind_to_raw(
            SyntaxKind::try_from(node.kind())
                .unwrap_or_else(|()| unreachable!("found unexpected node kind: {}", node.kind())),
        );

        if node.child_count() == 0 {
            return self.token(node);
        }

        self.builder.start_node(kind);

        for child in node.children(&mut node.walk()) {
            if child.is_named() {
                self.visit_node(child);
            } else if child.is_missing() {
                todo!()
            } else if child.is_error() {
                todo!()
            } else {
                self.token(child);
            }
        }
        self.builder.finish_node();
    }
}
