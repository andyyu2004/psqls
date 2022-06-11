use std::marker::PhantomData;

use rowan::{GreenNodeBuilder, Language, SyntaxNode};
use tree_sitter::{Node, Tree};

fn tree_sitter_to_rowan<L, K>(tree: Tree) -> SyntaxNode<L>
where
    L: Language,
    L::Kind: From<u16>,
{
    Builder::new().build(tree)
}

struct Builder<L> {
    builder: GreenNodeBuilder<'static>,
    _language: PhantomData<L>,
}

impl<L> Builder<L>
where
    L: Language,
    L::Kind: From<u16>,
{
    fn new() -> Self {
        Self {
            _language: PhantomData,
            builder: Default::default(),
        }
    }

    fn build(self, tree: Tree) -> SyntaxNode<L> {
        let node = tree.root_node();
        SyntaxNode::new_root(self.builder.finish())
    }

    fn visit_node(&mut self, node: Node) {
        let kind = L::kind_to_raw(L::Kind::from(node.kind_id()));
        self.builder.start_node(kind);
        for child in node.named_children(&mut node.walk()) {
            self.visit_node(child);
        }
        self.builder.finish_node();
    }
}
