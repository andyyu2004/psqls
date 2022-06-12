use std::sync::Arc;

use rowan::{GreenNode, GreenNodeBuilder, Language};
use tree_sitter::{Node, Tree};

use crate::{generated::SyntaxKind, node::Sql};

// the reason for using rowan at all is due to treesitter nodes having an inconvenient
// lifetime which doesn't work well with salsa
// You could also argue why we even bother with salsa for this..
pub(crate) fn ts_to_rowan(text: Arc<str>, tree: Tree) -> GreenNode {
    Builder::new(text).build(tree)
}

struct Builder {
    text: Arc<str>,
    builder: GreenNodeBuilder<'static>,
}

impl Builder {
    fn new(text: Arc<str>) -> Self {
        Self {
            text,
            builder: Default::default(),
        }
    }

    fn build(mut self, tree: Tree) -> GreenNode {
        let node = tree.root_node();
        self.visit_node(node);
        self.builder.finish()
    }

    fn visit_node(&mut self, node: Node) {
        let kind = Sql::kind_to_raw(
            SyntaxKind::try_from(node.kind())
                .unwrap_or_else(|()| unreachable!("found unexpected node kind: {}", node.kind())),
        );
        self.builder.start_node(kind);
        for child in node.children(&mut node.walk()) {
            if child.is_named() {
                self.visit_node(child);
            } else if child.is_missing() {
                todo!()
            } else if child.is_error() {
                todo!()
            } else {
                let kind = SyntaxKind::try_from(child.kind()).unwrap_or(SyntaxKind::Token);
                self.builder.token(
                    Sql::kind_to_raw(kind),
                    child.utf8_text(self.text.as_bytes()).unwrap(),
                );
            }
        }
        self.builder.finish_node();
    }
}
