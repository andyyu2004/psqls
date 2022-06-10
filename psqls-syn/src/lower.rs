use tree_sitter::{Node, Tree};

fn lower<'tree>(tree: &Tree) -> Node<'_> {
    tree.root_node()
}
