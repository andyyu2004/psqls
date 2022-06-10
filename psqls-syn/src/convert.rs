use std::marker::PhantomData;

use rowan::{GreenNodeBuilder, Language, SyntaxNode};
use tree_sitter::{Node, Tree};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
enum SyntaxKind {
    Todo,
}

// From `tree_sitter::Node::kind_id()`
impl From<u16> for SyntaxKind {
    fn from(u: u16) -> Self {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Sql {}

impl Language for Sql {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        unsafe { std::mem::transmute(raw) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        unsafe { std::mem::transmute(kind) }
    }
}

fn tree_sitter_to_rowan(tree: Tree) -> SyntaxNode<Sql> {
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
