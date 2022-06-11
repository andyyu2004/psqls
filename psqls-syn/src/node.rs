use crate::nodes::SyntaxKind;

// // can use this instead of the generated stuff if we need a to generate fresh
// #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
// #[repr(u16)]
// pub enum SyntaxKind {
//     _Z,
// }
// impl rowan::Language for Sql {
//     type Kind = SyntaxKind;
//     fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
//         unsafe { std::mem::transmute(raw) }
//     }
//     fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
//         unsafe { std::mem::transmute(kind) }
//     }
// }

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sql {}

pub type SyntaxNode = rowan::SyntaxNode<Sql>;
pub type GreenNode = rowan::GreenNode;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<Sql>;

pub trait Node {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized;
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized;
    fn syntax(&self) -> &SyntaxNode;
    fn children<C: Node>(&self) -> Children<C> {
        Children::new(self.syntax())
    }
    fn child<C: Node>(&self) -> Option<C> {
        self.syntax().children().find_map(C::cast)
    }
}

#[derive(Debug, Clone)]
pub struct Children<N> {
    inner: SyntaxNodeChildren,
    _type: std::marker::PhantomData<N>,
}

impl<N> Children<N> {
    fn new(parent: &SyntaxNode) -> Self {
        Children {
            inner: parent.children(),
            _type: std::marker::PhantomData,
        }
    }
}

impl<N: Node> Iterator for Children<N> {
    type Item = N;
    fn next(&mut self) -> Option<N> {
        self.inner.find_map(N::cast)
    }
}
