use std::marker::PhantomData;

use rowan::{GreenNode, GreenNodeBuilder, Language};
use tree_sitter::{Node, Tree};

// the reason for using rowan at all is due to treesitter nodes having an inconvenient
// lifetime which doesn't work well with salsa
// You could also argue why we even bother with salsa for this..
pub(crate) fn ts_to_rowan<L>(tree: Tree) -> GreenNode
where
    L: Language,
    L::Kind: From<&'static str>,
{
    Builder::<L>::new().build(tree)
}

struct Builder<L> {
    builder: GreenNodeBuilder<'static>,
    _language: PhantomData<L>,
}

impl<L> Builder<L>
where
    L: Language,
    L::Kind: From<&'static str>,
{
    fn new() -> Self {
        Self {
            _language: PhantomData,
            builder: Default::default(),
        }
    }

    fn build(self, tree: Tree) -> GreenNode {
        let node = tree.root_node();
        self.builder.finish()
    }

    fn visit_node(&mut self, node: Node) {
        let kind = L::kind_to_raw(L::Kind::from(node.kind()));
        self.builder.start_node(kind);
        for child in node.named_children(&mut node.walk()) {
            self.visit_node(child);
        }
        self.builder.finish_node();
    }
}
