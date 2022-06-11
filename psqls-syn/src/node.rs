use crate::generated::SyntaxKind;

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
