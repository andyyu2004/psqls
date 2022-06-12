//! generated, do not edit
#![allow(unused)]
use crate::node::*;
impl rowan::Language for Sql {
    type Kind = SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        unsafe { std::mem::transmute(raw) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        unsafe { std::mem::transmute(kind) }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct False(pub(crate) SyntaxNode);
impl Node for False {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::False
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl False {
    pub fn false_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::FalseKw)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Null(pub(crate) SyntaxNode);
impl Node for Null {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Null
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl Null {
    pub fn null_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::NullKw)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct True(pub(crate) SyntaxNode);
impl Node for True {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::True
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl True {
    pub fn true_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::TrueKw)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterStatement(pub(crate) SyntaxNode);
impl Node for AlterStatement {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::AlterStatement
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl AlterStatement {
    pub fn alter_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::AlterKw)
    }
}
impl AlterStatement {
    pub fn r#sequence(&self) -> Option<Sequence> {
        self.child()
    }
}
impl AlterStatement {
    pub fn r#alter_table(&self) -> Option<AlterTable> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterTable(pub(crate) SyntaxNode);
impl Node for AlterTable {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::AlterTable
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl AlterTable {
    pub fn table_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::TableKw)
    }
}
impl AlterTable {
    pub fn if_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::IfKw)
    }
}
impl AlterTable {
    pub fn exists_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::ExistsKw)
    }
}
impl AlterTable {
    pub fn only_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::OnlyKw)
    }
}
impl AlterTable {
    pub fn r#alter_table_action(&self) -> Option<AlterTableAction> {
        self.child()
    }
}
pub enum AlterTableAction {
    AlterTableActionAdd(AlterTableActionAdd),
    AlterTableActionAlterColumn(AlterTableActionAlterColumn),
}
impl Node for AlterTableAction {
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::AlterTableActionAdd | SyntaxKind::AlterTableActionAlterColumn
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        match syntax.kind() {
            SyntaxKind::AlterTableActionAdd => Some(AlterTableAction::AlterTableActionAdd(
                AlterTableActionAdd(syntax),
            )),
            SyntaxKind::AlterTableActionAlterColumn => Some(
                AlterTableAction::AlterTableActionAlterColumn(AlterTableActionAlterColumn(syntax)),
            ),
            _ => None,
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::AlterTableActionAdd(node) => node.syntax(),
            Self::AlterTableActionAlterColumn(node) => node.syntax(),
        }
    }
}
impl AlterTableAction {
    pub fn r#alter_table_action_add(&self) -> Option<AlterTableActionAdd> {
        self.child()
    }
}
impl AlterTableAction {
    pub fn r#alter_table_action_alter_column(&self) -> Option<AlterTableActionAlterColumn> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterTableActionAdd(pub(crate) SyntaxNode);
impl Node for AlterTableActionAdd {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::AlterTableActionAdd
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl AlterTableActionAdd {
    pub fn add_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::AddKw)
    }
}
impl AlterTableActionAdd {
    pub fn column_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::ColumnKw)
    }
}
impl AlterTableActionAdd {
    pub fn r#table_column(&self) -> Option<TableColumn> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterTableActionAlterColumn(pub(crate) SyntaxNode);
impl Node for AlterTableActionAlterColumn {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::AlterTableActionAlterColumn
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl AlterTableActionAlterColumn {
    pub fn alter_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::AlterKw)
    }
}
impl AlterTableActionAlterColumn {
    pub fn column_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::ColumnKw)
    }
}
impl AlterTableActionAlterColumn {
    pub fn set_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::SetKw)
    }
}
impl AlterTableActionAlterColumn {
    pub fn default_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::DefaultKw)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArgumentReference(pub(crate) SyntaxNode);
impl Node for ArgumentReference {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ArgumentReference
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayElementAccess(pub(crate) SyntaxNode);
impl Node for ArrayElementAccess {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ArrayElementAccess
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl ArrayElementAccess {
    pub fn r#identifier(&self) -> Option<Identifier> {
        self.child()
    }
}
impl ArrayElementAccess {
    pub fn r#argument_reference(&self) -> Option<ArgumentReference> {
        self.child()
    }
}
impl ArrayElementAccess {
    pub fn r#expression(&self) -> Option<Expression> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayType(pub(crate) SyntaxNode);
impl Node for ArrayType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ArrayType
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssigmentExpression(pub(crate) SyntaxNode);
impl Node for AssigmentExpression {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::AssigmentExpression
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl AssigmentExpression {
    pub fn r#identifier(&self) -> Option<Identifier> {
        self.child()
    }
}
impl AssigmentExpression {
    pub fn r#expression(&self) -> Option<Expression> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AsteriskExpression(pub(crate) SyntaxNode);
impl Node for AsteriskExpression {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::AsteriskExpression
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl AsteriskExpression {
    pub fn r#identifier(&self) -> Option<Identifier> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AutoIncrementConstraint(pub(crate) SyntaxNode);
impl Node for AutoIncrementConstraint {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::AutoIncrementConstraint
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl AutoIncrementConstraint {
    pub fn autoincrement_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::AutoincrementKw)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinaryExpression(pub(crate) SyntaxNode);
impl Node for BinaryExpression {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::BinaryExpression
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl BinaryExpression {
    pub fn r#expression(&self) -> Option<Expression> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinaryOperator(pub(crate) SyntaxNode);
impl Node for BinaryOperator {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::BinaryOperator
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BooleanExpression(pub(crate) SyntaxNode);
impl Node for BooleanExpression {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::BooleanExpression
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl BooleanExpression {
    pub fn not_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::NotKw)
    }
}
impl BooleanExpression {
    pub fn r#expression(&self) -> Option<Expression> {
        self.child()
    }
}
impl BooleanExpression {
    pub fn and_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::AndKw)
    }
}
impl BooleanExpression {
    pub fn or_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::OrKw)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CheckConstraint(pub(crate) SyntaxNode);
impl Node for CheckConstraint {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CheckConstraint
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl CheckConstraint {
    pub fn check_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::CheckKw)
    }
}
impl CheckConstraint {
    pub fn r#expression(&self) -> Option<Expression> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ColumnDefault(pub(crate) SyntaxNode);
impl Node for ColumnDefault {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ColumnDefault
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl ColumnDefault {
    pub fn default_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::DefaultKw)
    }
}
impl ColumnDefault {
    pub fn r#type_cast(&self) -> Option<TypeCast> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Comment(pub(crate) SyntaxNode);
impl Node for Comment {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Comment
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComparisonOperator(pub(crate) SyntaxNode);
impl Node for ComparisonOperator {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ComparisonOperator
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl ComparisonOperator {
    pub fn r#expression(&self) -> Option<Expression> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstrainedType(pub(crate) SyntaxNode);
impl Node for ConstrainedType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ConstrainedType
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl ConstrainedType {
    pub fn r#null_constraint(&self) -> Option<NullConstraint> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateDomainStatement(pub(crate) SyntaxNode);
impl Node for CreateDomainStatement {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CreateDomainStatement
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl CreateDomainStatement {
    pub fn create_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::CreateKw)
    }
}
impl CreateDomainStatement {
    pub fn domain_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::DomainKw)
    }
}
impl CreateDomainStatement {
    pub fn r#identifier(&self) -> Option<Identifier> {
        self.child()
    }
}
impl CreateDomainStatement {
    pub fn as_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::AsKw)
    }
}
impl CreateDomainStatement {
    pub fn r#null_constraint(&self) -> Option<NullConstraint> {
        self.child()
    }
}
impl CreateDomainStatement {
    pub fn r#check_constraint(&self) -> Option<CheckConstraint> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateExtensionStatement(pub(crate) SyntaxNode);
impl Node for CreateExtensionStatement {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CreateExtensionStatement
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl CreateExtensionStatement {
    pub fn create_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::CreateKw)
    }
}
impl CreateExtensionStatement {
    pub fn extension_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::ExtensionKw)
    }
}
impl CreateExtensionStatement {
    pub fn if_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::IfKw)
    }
}
impl CreateExtensionStatement {
    pub fn not_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::NotKw)
    }
}
impl CreateExtensionStatement {
    pub fn exists_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::ExistsKw)
    }
}
impl CreateExtensionStatement {
    pub fn r#identifier(&self) -> Option<Identifier> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateFunctionParameter(pub(crate) SyntaxNode);
impl Node for CreateFunctionParameter {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CreateFunctionParameter
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl CreateFunctionParameter {
    pub fn in_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::InKw)
    }
}
impl CreateFunctionParameter {
    pub fn out_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::OutKw)
    }
}
impl CreateFunctionParameter {
    pub fn inout_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::InoutKw)
    }
}
impl CreateFunctionParameter {
    pub fn variadic_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::VariadicKw)
    }
}
impl CreateFunctionParameter {
    pub fn r#identifier(&self) -> Option<Identifier> {
        self.child()
    }
}
impl CreateFunctionParameter {
    pub fn r#constrained_type(&self) -> Option<ConstrainedType> {
        self.child()
    }
}
impl CreateFunctionParameter {
    pub fn r#expression(&self) -> Option<Expression> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateFunctionParameters(pub(crate) SyntaxNode);
impl Node for CreateFunctionParameters {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CreateFunctionParameters
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl CreateFunctionParameters {
    pub fn create_function_parameters(&self) -> impl Iterator<Item = CreateFunctionParameter> {
        self.children()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateFunctionStatement(pub(crate) SyntaxNode);
impl Node for CreateFunctionStatement {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CreateFunctionStatement
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl CreateFunctionStatement {
    pub fn create_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::CreateKw)
    }
}
impl CreateFunctionStatement {
    pub fn orreplace_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::OrreplaceKw)
    }
}
impl CreateFunctionStatement {
    pub fn function_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::FunctionKw)
    }
}
impl CreateFunctionStatement {
    pub fn r#identifier(&self) -> Option<Identifier> {
        self.child()
    }
}
impl CreateFunctionStatement {
    pub fn r#create_function_parameters(&self) -> Option<CreateFunctionParameters> {
        self.child()
    }
}
impl CreateFunctionStatement {
    pub fn returns_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::ReturnsKw)
    }
}
impl CreateFunctionStatement {
    pub fn r#function_body(&self) -> Option<FunctionBody> {
        self.child()
    }
}
impl CreateFunctionStatement {
    pub fn r#optimizer_hint(&self) -> Option<OptimizerHint> {
        self.child()
    }
}
impl CreateFunctionStatement {
    pub fn r#parallel_hint(&self) -> Option<ParallelHint> {
        self.child()
    }
}
impl CreateFunctionStatement {
    pub fn r#null_hint(&self) -> Option<NullHint> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateIndexStatement(pub(crate) SyntaxNode);
impl Node for CreateIndexStatement {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CreateIndexStatement
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl CreateIndexStatement {
    pub fn create_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::CreateKw)
    }
}
impl CreateIndexStatement {
    pub fn r#unique_constraint(&self) -> Option<UniqueConstraint> {
        self.child()
    }
}
impl CreateIndexStatement {
    pub fn index_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::IndexKw)
    }
}
impl CreateIndexStatement {
    pub fn r#identifier(&self) -> Option<Identifier> {
        self.child()
    }
}
impl CreateIndexStatement {
    pub fn on_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::OnKw)
    }
}
impl CreateIndexStatement {
    pub fn r#using_clause(&self) -> Option<UsingClause> {
        self.child()
    }
}
impl CreateIndexStatement {
    pub fn r#index_table_parameters(&self) -> Option<IndexTableParameters> {
        self.child()
    }
}
impl CreateIndexStatement {
    pub fn r#where_clause(&self) -> Option<WhereClause> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateRoleStatement(pub(crate) SyntaxNode);
impl Node for CreateRoleStatement {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CreateRoleStatement
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl CreateRoleStatement {
    pub fn create_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::CreateKw)
    }
}
impl CreateRoleStatement {
    pub fn role_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::RoleKw)
    }
}
impl CreateRoleStatement {
    pub fn r#identifier(&self) -> Option<Identifier> {
        self.child()
    }
}
impl CreateRoleStatement {
    pub fn with_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::WithKw)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateSchemaStatement(pub(crate) SyntaxNode);
impl Node for CreateSchemaStatement {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CreateSchemaStatement
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl CreateSchemaStatement {
    pub fn create_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::CreateKw)
    }
}
impl CreateSchemaStatement {
    pub fn schema_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::SchemaKw)
    }
}
impl CreateSchemaStatement {
    pub fn if_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::IfKw)
    }
}
impl CreateSchemaStatement {
    pub fn not_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::NotKw)
    }
}
impl CreateSchemaStatement {
    pub fn exists_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::ExistsKw)
    }
}
impl CreateSchemaStatement {
    pub fn r#identifier(&self) -> Option<Identifier> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateStatement(pub(crate) SyntaxNode);
impl Node for CreateStatement {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CreateStatement
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl CreateStatement {
    pub fn create_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::CreateKw)
    }
}
impl CreateStatement {
    pub fn temp_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::TempKw)
    }
}
impl CreateStatement {
    pub fn temporary_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::TemporaryKw)
    }
}
impl CreateStatement {
    pub fn r#sequence(&self) -> Option<Sequence> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateTableStatement(pub(crate) SyntaxNode);
impl Node for CreateTableStatement {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CreateTableStatement
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl CreateTableStatement {
    pub fn create_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::CreateKw)
    }
}
impl CreateTableStatement {
    pub fn temporary_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::TemporaryKw)
    }
}
impl CreateTableStatement {
    pub fn table_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::TableKw)
    }
}
impl CreateTableStatement {
    pub fn if_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::IfKw)
    }
}
impl CreateTableStatement {
    pub fn not_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::NotKw)
    }
}
impl CreateTableStatement {
    pub fn exists_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::ExistsKw)
    }
}
impl CreateTableStatement {
    pub fn r#table_parameters(&self) -> Option<TableParameters> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateTypeStatement(pub(crate) SyntaxNode);
impl Node for CreateTypeStatement {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CreateTypeStatement
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl CreateTypeStatement {
    pub fn create_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::CreateKw)
    }
}
impl CreateTypeStatement {
    pub fn type_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::TypeKw)
    }
}
impl CreateTypeStatement {
    pub fn r#identifier(&self) -> Option<Identifier> {
        self.child()
    }
}
impl CreateTypeStatement {
    pub fn as_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::AsKw)
    }
}
impl CreateTypeStatement {
    pub fn r#parameters(&self) -> Option<Parameters> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DirectionConstraint(pub(crate) SyntaxNode);
impl Node for DirectionConstraint {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DirectionConstraint
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl DirectionConstraint {
    pub fn asc_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::AscKw)
    }
}
impl DirectionConstraint {
    pub fn desc_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::DescKw)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DistinctFrom(pub(crate) SyntaxNode);
impl Node for DistinctFrom {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DistinctFrom
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl DistinctFrom {
    pub fn distinct_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::DistinctKw)
    }
}
impl DistinctFrom {
    pub fn from_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::FromKw)
    }
}
impl DistinctFrom {
    pub fn r#expression(&self) -> Option<Expression> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DottedName(pub(crate) SyntaxNode);
impl Node for DottedName {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DottedName
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl DottedName {
    pub fn r#identifier(&self) -> Option<Identifier> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropStatement(pub(crate) SyntaxNode);
impl Node for DropStatement {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DropStatement
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl DropStatement {
    pub fn drop_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::DropKw)
    }
}
impl DropStatement {
    pub fn table_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::TableKw)
    }
}
impl DropStatement {
    pub fn view_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::ViewKw)
    }
}
impl DropStatement {
    pub fn tablespace_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::TablespaceKw)
    }
}
impl DropStatement {
    pub fn extension_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::ExtensionKw)
    }
}
impl DropStatement {
    pub fn index_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::IndexKw)
    }
}
impl DropStatement {
    pub fn if_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::IfKw)
    }
}
impl DropStatement {
    pub fn exists_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::ExistsKw)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExcludeEntry(pub(crate) SyntaxNode);
impl Node for ExcludeEntry {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ExcludeEntry
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl ExcludeEntry {
    pub fn r#op_class(&self) -> Option<OpClass> {
        self.child()
    }
}
impl ExcludeEntry {
    pub fn with_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::WithKw)
    }
}
impl ExcludeEntry {
    pub fn r#binary_operator(&self) -> Option<BinaryOperator> {
        self.child()
    }
}
pub enum Expression {
    IntervalExpression(IntervalExpression),
    FunctionCall(FunctionCall),
    String(String),
    FieldAccess(FieldAccess),
    True(True),
    False(False),
    Null(Null),
    AsteriskExpression(AsteriskExpression),
    Identifier(Identifier),
    Number(Number),
    ComparisonOperator(ComparisonOperator),
    InExpression(InExpression),
    IsExpression(IsExpression),
    BooleanExpression(BooleanExpression),
    ParenthesizedExpression(ParenthesizedExpression),
    TypeCast(TypeCast),
    BinaryExpression(BinaryExpression),
    ArrayElementAccess(ArrayElementAccess),
    ArgumentReference(ArgumentReference),
    SelectSubexpression(SelectSubexpression),
}
impl Node for Expression {
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::IntervalExpression
                | SyntaxKind::FunctionCall
                | SyntaxKind::String
                | SyntaxKind::FieldAccess
                | SyntaxKind::True
                | SyntaxKind::False
                | SyntaxKind::Null
                | SyntaxKind::AsteriskExpression
                | SyntaxKind::Identifier
                | SyntaxKind::Number
                | SyntaxKind::ComparisonOperator
                | SyntaxKind::InExpression
                | SyntaxKind::IsExpression
                | SyntaxKind::BooleanExpression
                | SyntaxKind::ParenthesizedExpression
                | SyntaxKind::TypeCast
                | SyntaxKind::BinaryExpression
                | SyntaxKind::ArrayElementAccess
                | SyntaxKind::ArgumentReference
                | SyntaxKind::SelectSubexpression
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        match syntax.kind() {
            SyntaxKind::IntervalExpression => {
                Some(Expression::IntervalExpression(IntervalExpression(syntax)))
            }
            SyntaxKind::FunctionCall => Some(Expression::FunctionCall(FunctionCall(syntax))),
            SyntaxKind::String => Some(Expression::String(String(syntax))),
            SyntaxKind::FieldAccess => Some(Expression::FieldAccess(FieldAccess(syntax))),
            SyntaxKind::True => Some(Expression::True(True(syntax))),
            SyntaxKind::False => Some(Expression::False(False(syntax))),
            SyntaxKind::Null => Some(Expression::Null(Null(syntax))),
            SyntaxKind::AsteriskExpression => {
                Some(Expression::AsteriskExpression(AsteriskExpression(syntax)))
            }
            SyntaxKind::Identifier => Some(Expression::Identifier(Identifier(syntax))),
            SyntaxKind::Number => Some(Expression::Number(Number(syntax))),
            SyntaxKind::ComparisonOperator => {
                Some(Expression::ComparisonOperator(ComparisonOperator(syntax)))
            }
            SyntaxKind::InExpression => Some(Expression::InExpression(InExpression(syntax))),
            SyntaxKind::IsExpression => Some(Expression::IsExpression(IsExpression(syntax))),
            SyntaxKind::BooleanExpression => {
                Some(Expression::BooleanExpression(BooleanExpression(syntax)))
            }
            SyntaxKind::ParenthesizedExpression => Some(Expression::ParenthesizedExpression(
                ParenthesizedExpression(syntax),
            )),
            SyntaxKind::TypeCast => Some(Expression::TypeCast(TypeCast(syntax))),
            SyntaxKind::BinaryExpression => {
                Some(Expression::BinaryExpression(BinaryExpression(syntax)))
            }
            SyntaxKind::ArrayElementAccess => {
                Some(Expression::ArrayElementAccess(ArrayElementAccess(syntax)))
            }
            SyntaxKind::ArgumentReference => {
                Some(Expression::ArgumentReference(ArgumentReference(syntax)))
            }
            SyntaxKind::SelectSubexpression => {
                Some(Expression::SelectSubexpression(SelectSubexpression(syntax)))
            }
            _ => None,
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::IntervalExpression(node) => node.syntax(),
            Self::FunctionCall(node) => node.syntax(),
            Self::String(node) => node.syntax(),
            Self::FieldAccess(node) => node.syntax(),
            Self::True(node) => node.syntax(),
            Self::False(node) => node.syntax(),
            Self::Null(node) => node.syntax(),
            Self::AsteriskExpression(node) => node.syntax(),
            Self::Identifier(node) => node.syntax(),
            Self::Number(node) => node.syntax(),
            Self::ComparisonOperator(node) => node.syntax(),
            Self::InExpression(node) => node.syntax(),
            Self::IsExpression(node) => node.syntax(),
            Self::BooleanExpression(node) => node.syntax(),
            Self::ParenthesizedExpression(node) => node.syntax(),
            Self::TypeCast(node) => node.syntax(),
            Self::BinaryExpression(node) => node.syntax(),
            Self::ArrayElementAccess(node) => node.syntax(),
            Self::ArgumentReference(node) => node.syntax(),
            Self::SelectSubexpression(node) => node.syntax(),
        }
    }
}
impl Expression {
    pub fn r#interval_expression(&self) -> Option<IntervalExpression> {
        self.child()
    }
}
impl Expression {
    pub fn r#function_call(&self) -> Option<FunctionCall> {
        self.child()
    }
}
impl Expression {
    pub fn r#string(&self) -> Option<String> {
        self.child()
    }
}
impl Expression {
    pub fn r#field_access(&self) -> Option<FieldAccess> {
        self.child()
    }
}
impl Expression {
    pub fn r#true(&self) -> Option<True> {
        self.child()
    }
}
impl Expression {
    pub fn r#false(&self) -> Option<False> {
        self.child()
    }
}
impl Expression {
    pub fn r#null(&self) -> Option<Null> {
        self.child()
    }
}
impl Expression {
    pub fn r#asterisk_expression(&self) -> Option<AsteriskExpression> {
        self.child()
    }
}
impl Expression {
    pub fn r#number(&self) -> Option<Number> {
        self.child()
    }
}
impl Expression {
    pub fn r#comparison_operator(&self) -> Option<ComparisonOperator> {
        self.child()
    }
}
impl Expression {
    pub fn r#in_expression(&self) -> Option<InExpression> {
        self.child()
    }
}
impl Expression {
    pub fn r#is_expression(&self) -> Option<IsExpression> {
        self.child()
    }
}
impl Expression {
    pub fn r#boolean_expression(&self) -> Option<BooleanExpression> {
        self.child()
    }
}
impl Expression {
    pub fn r#parenthesized_expression(&self) -> Option<ParenthesizedExpression> {
        self.child()
    }
}
impl Expression {
    pub fn r#type_cast(&self) -> Option<TypeCast> {
        self.child()
    }
}
impl Expression {
    pub fn r#binary_expression(&self) -> Option<BinaryExpression> {
        self.child()
    }
}
impl Expression {
    pub fn r#array_element_access(&self) -> Option<ArrayElementAccess> {
        self.child()
    }
}
impl Expression {
    pub fn r#argument_reference(&self) -> Option<ArgumentReference> {
        self.child()
    }
}
impl Expression {
    pub fn r#select_subexpression(&self) -> Option<SelectSubexpression> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldAccess(pub(crate) SyntaxNode);
impl Node for FieldAccess {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FieldAccess
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl FieldAccess {
    pub fn r#identifier(&self) -> Option<Identifier> {
        self.child()
    }
}
impl FieldAccess {
    pub fn r#string(&self) -> Option<String> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FromClause(pub(crate) SyntaxNode);
impl Node for FromClause {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FromClause
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl FromClause {
    pub fn from_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::FromKw)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionBody(pub(crate) SyntaxNode);
impl Node for FunctionBody {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FunctionBody
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl FunctionBody {
    pub fn as_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::AsKw)
    }
}
impl FunctionBody {
    pub fn r#select_statement(&self) -> Option<SelectStatement> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionCall(pub(crate) SyntaxNode);
impl Node for FunctionCall {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FunctionCall
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl FunctionCall {
    pub fn r#identifier(&self) -> Option<Identifier> {
        self.child()
    }
}
impl FunctionCall {
    pub fn expressions(&self) -> impl Iterator<Item = Expression> {
        self.children()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GrantStatement(pub(crate) SyntaxNode);
impl Node for GrantStatement {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::GrantStatement
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl GrantStatement {
    pub fn grant_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::GrantKw)
    }
}
impl GrantStatement {
    pub fn all_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::AllKw)
    }
}
impl GrantStatement {
    pub fn privileges_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::PrivilegesKw)
    }
}
impl GrantStatement {
    pub fn select_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::SelectKw)
    }
}
impl GrantStatement {
    pub fn insert_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::InsertKw)
    }
}
impl GrantStatement {
    pub fn update_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::UpdateKw)
    }
}
impl GrantStatement {
    pub fn delete_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::DeleteKw)
    }
}
impl GrantStatement {
    pub fn truncate_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::TruncateKw)
    }
}
impl GrantStatement {
    pub fn references_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::ReferencesKw)
    }
}
impl GrantStatement {
    pub fn trigger_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::TriggerKw)
    }
}
impl GrantStatement {
    pub fn usage_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::UsageKw)
    }
}
impl GrantStatement {
    pub fn on_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::OnKw)
    }
}
impl GrantStatement {
    pub fn schema_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::SchemaKw)
    }
}
impl GrantStatement {
    pub fn database_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::DatabaseKw)
    }
}
impl GrantStatement {
    pub fn sequence_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::SequenceKw)
    }
}
impl GrantStatement {
    pub fn table_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::TableKw)
    }
}
impl GrantStatement {
    pub fn r#identifier(&self) -> Option<Identifier> {
        self.child()
    }
}
impl GrantStatement {
    pub fn to_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::ToKw)
    }
}
impl GrantStatement {
    pub fn group_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::GroupKw)
    }
}
impl GrantStatement {
    pub fn public_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::PublicKw)
    }
}
impl GrantStatement {
    pub fn with_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::WithKw)
    }
}
impl GrantStatement {
    pub fn option_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::OptionKw)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GroupByClause(pub(crate) SyntaxNode);
impl Node for GroupByClause {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::GroupByClause
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl GroupByClause {
    pub fn group_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::GroupKw)
    }
}
impl GroupByClause {
    pub fn by_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::ByKw)
    }
}
impl GroupByClause {
    pub fn r#group_by_clause_body(&self) -> Option<GroupByClauseBody> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GroupByClauseBody(pub(crate) SyntaxNode);
impl Node for GroupByClauseBody {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::GroupByClauseBody
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl GroupByClauseBody {
    pub fn expressions(&self) -> impl Iterator<Item = Expression> {
        self.children()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(pub(crate) SyntaxNode);
impl Node for Identifier {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Identifier
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InExpression(pub(crate) SyntaxNode);
impl Node for InExpression {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::InExpression
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl InExpression {
    pub fn r#expression(&self) -> Option<Expression> {
        self.child()
    }
}
impl InExpression {
    pub fn not_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::NotKw)
    }
}
impl InExpression {
    pub fn in_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::InKw)
    }
}
impl InExpression {
    pub fn r#tuple(&self) -> Option<Tuple> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndexTableParameters(pub(crate) SyntaxNode);
impl Node for IndexTableParameters {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::IndexTableParameters
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl IndexTableParameters {
    pub fn r#expression(&self) -> Option<Expression> {
        self.child()
    }
}
impl IndexTableParameters {
    pub fn r#ordered_expression(&self) -> Option<OrderedExpression> {
        self.child()
    }
}
impl IndexTableParameters {
    pub fn r#op_class(&self) -> Option<OpClass> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InitialMode(pub(crate) SyntaxNode);
impl Node for InitialMode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::InitialMode
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl InitialMode {
    pub fn initially_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::InitiallyKw)
    }
}
impl InitialMode {
    pub fn deferred_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::DeferredKw)
    }
}
impl InitialMode {
    pub fn immediate_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::ImmediateKw)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InsertStatement(pub(crate) SyntaxNode);
impl Node for InsertStatement {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::InsertStatement
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl InsertStatement {
    pub fn insert_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::InsertKw)
    }
}
impl InsertStatement {
    pub fn into_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::IntoKw)
    }
}
impl InsertStatement {
    pub fn r#values_clause(&self) -> Option<ValuesClause> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntervalExpression(pub(crate) SyntaxNode);
impl Node for IntervalExpression {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::IntervalExpression
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl IntervalExpression {
    pub fn interval_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::IntervalKw)
    }
}
impl IntervalExpression {
    pub fn r#string(&self) -> Option<String> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IsExpression(pub(crate) SyntaxNode);
impl Node for IsExpression {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::IsExpression
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl IsExpression {
    pub fn r#expression(&self) -> Option<Expression> {
        self.child()
    }
}
impl IsExpression {
    pub fn is_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::IsKw)
    }
}
impl IsExpression {
    pub fn not_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::NotKw)
    }
}
impl IsExpression {
    pub fn r#null(&self) -> Option<Null> {
        self.child()
    }
}
impl IsExpression {
    pub fn r#true(&self) -> Option<True> {
        self.child()
    }
}
impl IsExpression {
    pub fn r#false(&self) -> Option<False> {
        self.child()
    }
}
impl IsExpression {
    pub fn r#distinct_from(&self) -> Option<DistinctFrom> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JoinClause(pub(crate) SyntaxNode);
impl Node for JoinClause {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::JoinClause
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl JoinClause {
    pub fn r#join_type(&self) -> Option<JoinType> {
        self.child()
    }
}
impl JoinClause {
    pub fn join_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::JoinKw)
    }
}
impl JoinClause {
    pub fn on_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::OnKw)
    }
}
impl JoinClause {
    pub fn r#expression(&self) -> Option<Expression> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JoinType(pub(crate) SyntaxNode);
impl Node for JoinType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::JoinType
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl JoinType {
    pub fn inner_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::InnerKw)
    }
}
impl JoinType {
    pub fn left_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::LeftKw)
    }
}
impl JoinType {
    pub fn right_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::RightKw)
    }
}
impl JoinType {
    pub fn full_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::FullKw)
    }
}
impl JoinType {
    pub fn outer_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::OuterKw)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Mode(pub(crate) SyntaxNode);
impl Node for Mode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Mode
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl Mode {
    pub fn not_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::NotKw)
    }
}
impl Mode {
    pub fn deferrable_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::DeferrableKw)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamedConstraint(pub(crate) SyntaxNode);
impl Node for NamedConstraint {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NamedConstraint
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl NamedConstraint {
    pub fn constraint_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::ConstraintKw)
    }
}
impl NamedConstraint {
    pub fn r#identifier(&self) -> Option<Identifier> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NullConstraint(pub(crate) SyntaxNode);
impl Node for NullConstraint {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NullConstraint
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl NullConstraint {
    pub fn not_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::NotKw)
    }
}
impl NullConstraint {
    pub fn r#null(&self) -> Option<Null> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NullHint(pub(crate) SyntaxNode);
impl Node for NullHint {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NullHint
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl NullHint {
    pub fn called_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::CalledKw)
    }
}
impl NullHint {
    pub fn on_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::OnKw)
    }
}
impl NullHint {
    pub fn null_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::NullKw)
    }
}
impl NullHint {
    pub fn input_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::InputKw)
    }
}
impl NullHint {
    pub fn returns_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::ReturnsKw)
    }
}
impl NullHint {
    pub fn strict_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::StrictKw)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Number(pub(crate) SyntaxNode);
impl Node for Number {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Number
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OnDeleteAction(pub(crate) SyntaxNode);
impl Node for OnDeleteAction {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::OnDeleteAction
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl OnDeleteAction {
    pub fn on_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::OnKw)
    }
}
impl OnDeleteAction {
    pub fn delete_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::DeleteKw)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OnUpdateAction(pub(crate) SyntaxNode);
impl Node for OnUpdateAction {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::OnUpdateAction
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl OnUpdateAction {
    pub fn on_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::OnKw)
    }
}
impl OnUpdateAction {
    pub fn update_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::UpdateKw)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OpClass(pub(crate) SyntaxNode);
impl Node for OpClass {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::OpClass
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OptimizerHint(pub(crate) SyntaxNode);
impl Node for OptimizerHint {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::OptimizerHint
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl OptimizerHint {
    pub fn volatile_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::VolatileKw)
    }
}
impl OptimizerHint {
    pub fn immutable_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::ImmutableKw)
    }
}
impl OptimizerHint {
    pub fn stable_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::StableKw)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderByClause(pub(crate) SyntaxNode);
impl Node for OrderByClause {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::OrderByClause
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl OrderByClause {
    pub fn order_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::OrderKw)
    }
}
impl OrderByClause {
    pub fn by_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::ByKw)
    }
}
impl OrderByClause {
    pub fn r#order_by_clause_body(&self) -> Option<OrderByClauseBody> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderByClauseBody(pub(crate) SyntaxNode);
impl Node for OrderByClauseBody {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::OrderByClauseBody
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl OrderByClauseBody {
    pub fn expressions(&self) -> impl Iterator<Item = Expression> {
        self.children()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderedExpression(pub(crate) SyntaxNode);
impl Node for OrderedExpression {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::OrderedExpression
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl OrderedExpression {
    pub fn r#expression(&self) -> Option<Expression> {
        self.child()
    }
}
impl OrderedExpression {
    pub fn asc_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::AscKw)
    }
}
impl OrderedExpression {
    pub fn desc_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::DescKw)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParallelHint(pub(crate) SyntaxNode);
impl Node for ParallelHint {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ParallelHint
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl ParallelHint {
    pub fn parallel_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::ParallelKw)
    }
}
impl ParallelHint {
    pub fn safe_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::SafeKw)
    }
}
impl ParallelHint {
    pub fn unsafe_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::UnsafeKw)
    }
}
impl ParallelHint {
    pub fn restricted_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::RestrictedKw)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Parameter(pub(crate) SyntaxNode);
impl Node for Parameter {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Parameter
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl Parameter {
    pub fn r#identifier(&self) -> Option<Identifier> {
        self.child()
    }
}
impl Parameter {
    pub fn r#constrained_type(&self) -> Option<ConstrainedType> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Parameters(pub(crate) SyntaxNode);
impl Node for Parameters {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Parameters
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl Parameters {
    pub fn parameters(&self) -> impl Iterator<Item = Parameter> {
        self.children()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParenthesizedExpression(pub(crate) SyntaxNode);
impl Node for ParenthesizedExpression {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ParenthesizedExpression
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl ParenthesizedExpression {
    pub fn r#expression(&self) -> Option<Expression> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PgCommand(pub(crate) SyntaxNode);
impl Node for PgCommand {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PgCommand
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrimaryKeyConstraint(pub(crate) SyntaxNode);
impl Node for PrimaryKeyConstraint {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PrimaryKeyConstraint
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl PrimaryKeyConstraint {
    pub fn primary_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::PrimaryKw)
    }
}
impl PrimaryKeyConstraint {
    pub fn key_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::KeyKw)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReferencesConstraint(pub(crate) SyntaxNode);
impl Node for ReferencesConstraint {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ReferencesConstraint
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl ReferencesConstraint {
    pub fn references_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::ReferencesKw)
    }
}
impl ReferencesConstraint {
    pub fn identifiers(&self) -> impl Iterator<Item = Identifier> {
        self.children()
    }
}
impl ReferencesConstraint {
    pub fn r#on_update_action(&self) -> Option<OnUpdateAction> {
        self.child()
    }
}
impl ReferencesConstraint {
    pub fn r#on_delete_action(&self) -> Option<OnDeleteAction> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SelectClause(pub(crate) SyntaxNode);
impl Node for SelectClause {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SelectClause
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl SelectClause {
    pub fn select_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::SelectKw)
    }
}
impl SelectClause {
    pub fn r#select_clause_body(&self) -> Option<SelectClauseBody> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SelectClauseBody(pub(crate) SyntaxNode);
impl Node for SelectClauseBody {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SelectClauseBody
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SelectStatement(pub(crate) SyntaxNode);
impl Node for SelectStatement {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SelectStatement
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl SelectStatement {
    pub fn r#select_clause(&self) -> Option<SelectClause> {
        self.child()
    }
}
impl SelectStatement {
    pub fn r#from_clause(&self) -> Option<FromClause> {
        self.child()
    }
}
impl SelectStatement {
    pub fn join_clauses(&self) -> impl Iterator<Item = JoinClause> {
        self.children()
    }
}
impl SelectStatement {
    pub fn r#where_clause(&self) -> Option<WhereClause> {
        self.child()
    }
}
impl SelectStatement {
    pub fn r#group_by_clause(&self) -> Option<GroupByClause> {
        self.child()
    }
}
impl SelectStatement {
    pub fn r#order_by_clause(&self) -> Option<OrderByClause> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SelectSubexpression(pub(crate) SyntaxNode);
impl Node for SelectSubexpression {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SelectSubexpression
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl SelectSubexpression {
    pub fn r#select_statement(&self) -> Option<SelectStatement> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sequence(pub(crate) SyntaxNode);
impl Node for Sequence {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Sequence
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl Sequence {
    pub fn sequence_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::SequenceKw)
    }
}
impl Sequence {
    pub fn if_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::IfKw)
    }
}
impl Sequence {
    pub fn not_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::NotKw)
    }
}
impl Sequence {
    pub fn exists_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::ExistsKw)
    }
}
impl Sequence {
    pub fn as_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::AsKw)
    }
}
impl Sequence {
    pub fn r#type(&self) -> Option<Type> {
        self.child()
    }
}
impl Sequence {
    pub fn start_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::StartKw)
    }
}
impl Sequence {
    pub fn with_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::WithKw)
    }
}
impl Sequence {
    pub fn r#number(&self) -> Option<Number> {
        self.child()
    }
}
impl Sequence {
    pub fn increment_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::IncrementKw)
    }
}
impl Sequence {
    pub fn by_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::ByKw)
    }
}
impl Sequence {
    pub fn no_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::NoKw)
    }
}
impl Sequence {
    pub fn minvalue_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::MinvalueKw)
    }
}
impl Sequence {
    pub fn maxvalue_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::MaxvalueKw)
    }
}
impl Sequence {
    pub fn cache_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::CacheKw)
    }
}
impl Sequence {
    pub fn owned_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::OwnedKw)
    }
}
impl Sequence {
    pub fn r#dotted_name(&self) -> Option<DottedName> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetClause(pub(crate) SyntaxNode);
impl Node for SetClause {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SetClause
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl SetClause {
    pub fn set_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::SetKw)
    }
}
impl SetClause {
    pub fn r#set_clause_body(&self) -> Option<SetClauseBody> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetClauseBody(pub(crate) SyntaxNode);
impl Node for SetClauseBody {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SetClauseBody
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl SetClauseBody {
    pub fn assigment_expressions(&self) -> impl Iterator<Item = AssigmentExpression> {
        self.children()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetStatement(pub(crate) SyntaxNode);
impl Node for SetStatement {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SetStatement
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl SetStatement {
    pub fn set_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::SetKw)
    }
}
impl SetStatement {
    pub fn session_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::SessionKw)
    }
}
impl SetStatement {
    pub fn local_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::LocalKw)
    }
}
impl SetStatement {
    pub fn r#identifier(&self) -> Option<Identifier> {
        self.child()
    }
}
impl SetStatement {
    pub fn to_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::ToKw)
    }
}
impl SetStatement {
    pub fn r#expression(&self) -> Option<Expression> {
        self.child()
    }
}
impl SetStatement {
    pub fn default_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::DefaultKw)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Setof(pub(crate) SyntaxNode);
impl Node for Setof {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Setof
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl Setof {
    pub fn setof_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::SetofKw)
    }
}
impl Setof {
    pub fn r#constrained_type(&self) -> Option<ConstrainedType> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceFile(pub(crate) SyntaxNode);
impl Node for SourceFile {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SourceFile
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl SourceFile {
    pub fn statements(&self) -> impl Iterator<Item = Statement> {
        self.children()
    }
}
pub enum Statement {
    PgCommand(PgCommand),
    SelectStatement(SelectStatement),
    UpdateStatement(UpdateStatement),
    SetStatement(SetStatement),
    InsertStatement(InsertStatement),
    GrantStatement(GrantStatement),
    DropStatement(DropStatement),
    CreateStatement(CreateStatement),
    AlterStatement(AlterStatement),
    CreateTypeStatement(CreateTypeStatement),
    CreateDomainStatement(CreateDomainStatement),
    CreateIndexStatement(CreateIndexStatement),
    CreateTableStatement(CreateTableStatement),
    CreateFunctionStatement(CreateFunctionStatement),
    CreateSchemaStatement(CreateSchemaStatement),
    CreateRoleStatement(CreateRoleStatement),
    CreateExtensionStatement(CreateExtensionStatement),
}
impl Node for Statement {
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::PgCommand
                | SyntaxKind::SelectStatement
                | SyntaxKind::UpdateStatement
                | SyntaxKind::SetStatement
                | SyntaxKind::InsertStatement
                | SyntaxKind::GrantStatement
                | SyntaxKind::DropStatement
                | SyntaxKind::CreateStatement
                | SyntaxKind::AlterStatement
                | SyntaxKind::CreateTypeStatement
                | SyntaxKind::CreateDomainStatement
                | SyntaxKind::CreateIndexStatement
                | SyntaxKind::CreateTableStatement
                | SyntaxKind::CreateFunctionStatement
                | SyntaxKind::CreateSchemaStatement
                | SyntaxKind::CreateRoleStatement
                | SyntaxKind::CreateExtensionStatement
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        match syntax.kind() {
            SyntaxKind::PgCommand => Some(Statement::PgCommand(PgCommand(syntax))),
            SyntaxKind::SelectStatement => {
                Some(Statement::SelectStatement(SelectStatement(syntax)))
            }
            SyntaxKind::UpdateStatement => {
                Some(Statement::UpdateStatement(UpdateStatement(syntax)))
            }
            SyntaxKind::SetStatement => Some(Statement::SetStatement(SetStatement(syntax))),
            SyntaxKind::InsertStatement => {
                Some(Statement::InsertStatement(InsertStatement(syntax)))
            }
            SyntaxKind::GrantStatement => Some(Statement::GrantStatement(GrantStatement(syntax))),
            SyntaxKind::DropStatement => Some(Statement::DropStatement(DropStatement(syntax))),
            SyntaxKind::CreateStatement => {
                Some(Statement::CreateStatement(CreateStatement(syntax)))
            }
            SyntaxKind::AlterStatement => Some(Statement::AlterStatement(AlterStatement(syntax))),
            SyntaxKind::CreateTypeStatement => {
                Some(Statement::CreateTypeStatement(CreateTypeStatement(syntax)))
            }
            SyntaxKind::CreateDomainStatement => Some(Statement::CreateDomainStatement(
                CreateDomainStatement(syntax),
            )),
            SyntaxKind::CreateIndexStatement => Some(Statement::CreateIndexStatement(
                CreateIndexStatement(syntax),
            )),
            SyntaxKind::CreateTableStatement => Some(Statement::CreateTableStatement(
                CreateTableStatement(syntax),
            )),
            SyntaxKind::CreateFunctionStatement => Some(Statement::CreateFunctionStatement(
                CreateFunctionStatement(syntax),
            )),
            SyntaxKind::CreateSchemaStatement => Some(Statement::CreateSchemaStatement(
                CreateSchemaStatement(syntax),
            )),
            SyntaxKind::CreateRoleStatement => {
                Some(Statement::CreateRoleStatement(CreateRoleStatement(syntax)))
            }
            SyntaxKind::CreateExtensionStatement => Some(Statement::CreateExtensionStatement(
                CreateExtensionStatement(syntax),
            )),
            _ => None,
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::PgCommand(node) => node.syntax(),
            Self::SelectStatement(node) => node.syntax(),
            Self::UpdateStatement(node) => node.syntax(),
            Self::SetStatement(node) => node.syntax(),
            Self::InsertStatement(node) => node.syntax(),
            Self::GrantStatement(node) => node.syntax(),
            Self::DropStatement(node) => node.syntax(),
            Self::CreateStatement(node) => node.syntax(),
            Self::AlterStatement(node) => node.syntax(),
            Self::CreateTypeStatement(node) => node.syntax(),
            Self::CreateDomainStatement(node) => node.syntax(),
            Self::CreateIndexStatement(node) => node.syntax(),
            Self::CreateTableStatement(node) => node.syntax(),
            Self::CreateFunctionStatement(node) => node.syntax(),
            Self::CreateSchemaStatement(node) => node.syntax(),
            Self::CreateRoleStatement(node) => node.syntax(),
            Self::CreateExtensionStatement(node) => node.syntax(),
        }
    }
}
impl Statement {
    pub fn r#pg_command(&self) -> Option<PgCommand> {
        self.child()
    }
}
impl Statement {
    pub fn r#select_statement(&self) -> Option<SelectStatement> {
        self.child()
    }
}
impl Statement {
    pub fn r#update_statement(&self) -> Option<UpdateStatement> {
        self.child()
    }
}
impl Statement {
    pub fn r#set_statement(&self) -> Option<SetStatement> {
        self.child()
    }
}
impl Statement {
    pub fn r#insert_statement(&self) -> Option<InsertStatement> {
        self.child()
    }
}
impl Statement {
    pub fn r#grant_statement(&self) -> Option<GrantStatement> {
        self.child()
    }
}
impl Statement {
    pub fn r#drop_statement(&self) -> Option<DropStatement> {
        self.child()
    }
}
impl Statement {
    pub fn r#create_statement(&self) -> Option<CreateStatement> {
        self.child()
    }
}
impl Statement {
    pub fn r#alter_statement(&self) -> Option<AlterStatement> {
        self.child()
    }
}
impl Statement {
    pub fn r#create_type_statement(&self) -> Option<CreateTypeStatement> {
        self.child()
    }
}
impl Statement {
    pub fn r#create_domain_statement(&self) -> Option<CreateDomainStatement> {
        self.child()
    }
}
impl Statement {
    pub fn r#create_index_statement(&self) -> Option<CreateIndexStatement> {
        self.child()
    }
}
impl Statement {
    pub fn r#create_table_statement(&self) -> Option<CreateTableStatement> {
        self.child()
    }
}
impl Statement {
    pub fn r#create_function_statement(&self) -> Option<CreateFunctionStatement> {
        self.child()
    }
}
impl Statement {
    pub fn r#create_schema_statement(&self) -> Option<CreateSchemaStatement> {
        self.child()
    }
}
impl Statement {
    pub fn r#create_role_statement(&self) -> Option<CreateRoleStatement> {
        self.child()
    }
}
impl Statement {
    pub fn r#create_extension_statement(&self) -> Option<CreateExtensionStatement> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct String(pub(crate) SyntaxNode);
impl Node for String {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::String
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableColumn(pub(crate) SyntaxNode);
impl Node for TableColumn {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TableColumn
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl TableColumn {
    pub fn r#column_default(&self) -> Option<ColumnDefault> {
        self.child()
    }
}
impl TableColumn {
    pub fn r#primary_key_constraint(&self) -> Option<PrimaryKeyConstraint> {
        self.child()
    }
}
impl TableColumn {
    pub fn r#check_constraint(&self) -> Option<CheckConstraint> {
        self.child()
    }
}
impl TableColumn {
    pub fn r#references_constraint(&self) -> Option<ReferencesConstraint> {
        self.child()
    }
}
impl TableColumn {
    pub fn r#unique_constraint(&self) -> Option<UniqueConstraint> {
        self.child()
    }
}
impl TableColumn {
    pub fn r#null_constraint(&self) -> Option<NullConstraint> {
        self.child()
    }
}
impl TableColumn {
    pub fn r#named_constraint(&self) -> Option<NamedConstraint> {
        self.child()
    }
}
impl TableColumn {
    pub fn r#direction_constraint(&self) -> Option<DirectionConstraint> {
        self.child()
    }
}
impl TableColumn {
    pub fn r#auto_increment_constraint(&self) -> Option<AutoIncrementConstraint> {
        self.child()
    }
}
impl TableColumn {
    pub fn r#time_zone_constraint(&self) -> Option<TimeZoneConstraint> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableConstraintCheck(pub(crate) SyntaxNode);
impl Node for TableConstraintCheck {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TableConstraintCheck
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl TableConstraintCheck {
    pub fn check_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::CheckKw)
    }
}
impl TableConstraintCheck {
    pub fn r#expression(&self) -> Option<Expression> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableConstraintExclude(pub(crate) SyntaxNode);
impl Node for TableConstraintExclude {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TableConstraintExclude
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl TableConstraintExclude {
    pub fn exclude_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::ExcludeKw)
    }
}
impl TableConstraintExclude {
    pub fn using_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::UsingKw)
    }
}
impl TableConstraintExclude {
    pub fn exclude_entrys(&self) -> impl Iterator<Item = ExcludeEntry> {
        self.children()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableConstraintForeignKey(pub(crate) SyntaxNode);
impl Node for TableConstraintForeignKey {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TableConstraintForeignKey
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl TableConstraintForeignKey {
    pub fn foreign_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::ForeignKw)
    }
}
impl TableConstraintForeignKey {
    pub fn key_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::KeyKw)
    }
}
impl TableConstraintForeignKey {
    pub fn identifiers(&self) -> impl Iterator<Item = Identifier> {
        self.children()
    }
}
impl TableConstraintForeignKey {
    pub fn r#references_constraint(&self) -> Option<ReferencesConstraint> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableConstraintPrimaryKey(pub(crate) SyntaxNode);
impl Node for TableConstraintPrimaryKey {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TableConstraintPrimaryKey
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl TableConstraintPrimaryKey {
    pub fn primary_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::PrimaryKw)
    }
}
impl TableConstraintPrimaryKey {
    pub fn key_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::KeyKw)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableConstraintUnique(pub(crate) SyntaxNode);
impl Node for TableConstraintUnique {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TableConstraintUnique
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl TableConstraintUnique {
    pub fn unique_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::UniqueKw)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableParameters(pub(crate) SyntaxNode);
impl Node for TableParameters {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TableParameters
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl TableParameters {
    pub fn r#table_column(&self) -> Option<TableColumn> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TimeZoneConstraint(pub(crate) SyntaxNode);
impl Node for TimeZoneConstraint {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TimeZoneConstraint
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl TimeZoneConstraint {
    pub fn with_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::WithKw)
    }
}
impl TimeZoneConstraint {
    pub fn without_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::WithoutKw)
    }
}
impl TimeZoneConstraint {
    pub fn time_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::TimeKw)
    }
}
impl TimeZoneConstraint {
    pub fn zone_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::ZoneKw)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tuple(pub(crate) SyntaxNode);
impl Node for Tuple {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Tuple
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl Tuple {
    pub fn expressions(&self) -> impl Iterator<Item = Expression> {
        self.children()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Type(pub(crate) SyntaxNode);
impl Node for Type {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Type
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl Type {
    pub fn r#number(&self) -> Option<Number> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeCast(pub(crate) SyntaxNode);
impl Node for TypeCast {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TypeCast
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl TypeCast {
    pub fn r#parenthesized_expression(&self) -> Option<ParenthesizedExpression> {
        self.child()
    }
}
impl TypeCast {
    pub fn r#string(&self) -> Option<String> {
        self.child()
    }
}
impl TypeCast {
    pub fn r#identifier(&self) -> Option<Identifier> {
        self.child()
    }
}
impl TypeCast {
    pub fn r#function_call(&self) -> Option<FunctionCall> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UniqueConstraint(pub(crate) SyntaxNode);
impl Node for UniqueConstraint {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::UniqueConstraint
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl UniqueConstraint {
    pub fn unique_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::UniqueKw)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UpdateStatement(pub(crate) SyntaxNode);
impl Node for UpdateStatement {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::UpdateStatement
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl UpdateStatement {
    pub fn update_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::UpdateKw)
    }
}
impl UpdateStatement {
    pub fn r#identifier(&self) -> Option<Identifier> {
        self.child()
    }
}
impl UpdateStatement {
    pub fn r#set_clause(&self) -> Option<SetClause> {
        self.child()
    }
}
impl UpdateStatement {
    pub fn r#where_clause(&self) -> Option<WhereClause> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UsingClause(pub(crate) SyntaxNode);
impl Node for UsingClause {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::UsingClause
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl UsingClause {
    pub fn using_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::UsingKw)
    }
}
impl UsingClause {
    pub fn r#identifier(&self) -> Option<Identifier> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValuesClause(pub(crate) SyntaxNode);
impl Node for ValuesClause {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ValuesClause
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl ValuesClause {
    pub fn values_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::ValuesKw)
    }
}
impl ValuesClause {
    pub fn r#values_clause_body(&self) -> Option<ValuesClauseBody> {
        self.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValuesClauseBody(pub(crate) SyntaxNode);
impl Node for ValuesClauseBody {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ValuesClauseBody
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl ValuesClauseBody {
    pub fn expressions(&self) -> impl Iterator<Item = Expression> {
        self.children()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhereClause(pub(crate) SyntaxNode);
impl Node for WhereClause {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::WhereClause
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl WhereClause {
    pub fn where_kw(&self) -> Option<SyntaxToken> {
        self.token(SyntaxKind::WhereKw)
    }
}
impl WhereClause {
    pub fn r#expression(&self) -> Option<Expression> {
        self.child()
    }
}
pub trait Visitor {
    fn visit_kw(&mut self, kw: SyntaxToken) {}
    fn visit_alter_statement(&mut self, r#alter_statement: AlterStatement) {
        if let Some(kw) = r#alter_statement.alter_kw() {
            self.visit_kw(kw);
        }
        for r#alter_table in r#alter_statement.r#alter_table() {
            self.visit_alter_table(r#alter_table);
        }
        for r#sequence in r#alter_statement.r#sequence() {
            self.visit_sequence(r#sequence);
        }
    }
    fn visit_alter_table(&mut self, r#alter_table: AlterTable) {
        if let Some(kw) = r#alter_table.exists_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#alter_table.if_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#alter_table.only_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#alter_table.table_kw() {
            self.visit_kw(kw);
        }
        for r#alter_table_action in r#alter_table.r#alter_table_action() {
            self.visit_alter_table_action(r#alter_table_action);
        }
    }
    fn visit_alter_table_action(&mut self, r#alter_table_action: AlterTableAction) {
        for r#alter_table_action_add in r#alter_table_action.r#alter_table_action_add() {
            self.visit_alter_table_action_add(r#alter_table_action_add);
        }
        for r#alter_table_action_alter_column in
            r#alter_table_action.r#alter_table_action_alter_column()
        {
            self.visit_alter_table_action_alter_column(r#alter_table_action_alter_column);
        }
    }
    fn visit_alter_table_action_add(&mut self, r#alter_table_action_add: AlterTableActionAdd) {
        if let Some(kw) = r#alter_table_action_add.add_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#alter_table_action_add.column_kw() {
            self.visit_kw(kw);
        }
        for r#table_column in r#alter_table_action_add.r#table_column() {
            self.visit_table_column(r#table_column);
        }
    }
    fn visit_alter_table_action_alter_column(
        &mut self,
        r#alter_table_action_alter_column: AlterTableActionAlterColumn,
    ) {
        if let Some(kw) = r#alter_table_action_alter_column.alter_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#alter_table_action_alter_column.column_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#alter_table_action_alter_column.default_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#alter_table_action_alter_column.set_kw() {
            self.visit_kw(kw);
        }
    }
    fn visit_argument_reference(&mut self, r#argument_reference: ArgumentReference) {}
    fn visit_array_element_access(&mut self, r#array_element_access: ArrayElementAccess) {
        for r#argument_reference in r#array_element_access.r#argument_reference() {
            self.visit_argument_reference(r#argument_reference);
        }
        for r#expression in r#array_element_access.r#expression() {
            self.visit_expression(r#expression);
        }
        for r#identifier in r#array_element_access.r#identifier() {
            self.visit_identifier(r#identifier);
        }
    }
    fn visit_array_type(&mut self, r#array_type: ArrayType) {}
    fn visit_assigment_expression(&mut self, r#assigment_expression: AssigmentExpression) {
        for r#expression in r#assigment_expression.r#expression() {
            self.visit_expression(r#expression);
        }
        for r#identifier in r#assigment_expression.r#identifier() {
            self.visit_identifier(r#identifier);
        }
    }
    fn visit_asterisk_expression(&mut self, r#asterisk_expression: AsteriskExpression) {
        for r#identifier in r#asterisk_expression.r#identifier() {
            self.visit_identifier(r#identifier);
        }
    }
    fn visit_auto_increment_constraint(
        &mut self,
        r#auto_increment_constraint: AutoIncrementConstraint,
    ) {
        if let Some(kw) = r#auto_increment_constraint.autoincrement_kw() {
            self.visit_kw(kw);
        }
    }
    fn visit_binary_expression(&mut self, r#binary_expression: BinaryExpression) {
        for r#expression in r#binary_expression.r#expression() {
            self.visit_expression(r#expression);
        }
    }
    fn visit_binary_operator(&mut self, r#binary_operator: BinaryOperator) {}
    fn visit_boolean_expression(&mut self, r#boolean_expression: BooleanExpression) {
        if let Some(kw) = r#boolean_expression.and_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#boolean_expression.not_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#boolean_expression.or_kw() {
            self.visit_kw(kw);
        }
        for r#expression in r#boolean_expression.r#expression() {
            self.visit_expression(r#expression);
        }
    }
    fn visit_check_constraint(&mut self, r#check_constraint: CheckConstraint) {
        if let Some(kw) = r#check_constraint.check_kw() {
            self.visit_kw(kw);
        }
        for r#expression in r#check_constraint.r#expression() {
            self.visit_expression(r#expression);
        }
    }
    fn visit_column_default(&mut self, r#column_default: ColumnDefault) {
        if let Some(kw) = r#column_default.default_kw() {
            self.visit_kw(kw);
        }
        for r#type_cast in r#column_default.r#type_cast() {
            self.visit_type_cast(r#type_cast);
        }
    }
    fn visit_comment(&mut self, r#comment: Comment) {}
    fn visit_comparison_operator(&mut self, r#comparison_operator: ComparisonOperator) {
        for r#expression in r#comparison_operator.r#expression() {
            self.visit_expression(r#expression);
        }
    }
    fn visit_constrained_type(&mut self, r#constrained_type: ConstrainedType) {
        for r#null_constraint in r#constrained_type.r#null_constraint() {
            self.visit_null_constraint(r#null_constraint);
        }
    }
    fn visit_create_domain_statement(&mut self, r#create_domain_statement: CreateDomainStatement) {
        if let Some(kw) = r#create_domain_statement.as_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#create_domain_statement.create_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#create_domain_statement.domain_kw() {
            self.visit_kw(kw);
        }
        for r#check_constraint in r#create_domain_statement.r#check_constraint() {
            self.visit_check_constraint(r#check_constraint);
        }
        for r#identifier in r#create_domain_statement.r#identifier() {
            self.visit_identifier(r#identifier);
        }
        for r#null_constraint in r#create_domain_statement.r#null_constraint() {
            self.visit_null_constraint(r#null_constraint);
        }
    }
    fn visit_create_extension_statement(
        &mut self,
        r#create_extension_statement: CreateExtensionStatement,
    ) {
        if let Some(kw) = r#create_extension_statement.create_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#create_extension_statement.exists_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#create_extension_statement.extension_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#create_extension_statement.if_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#create_extension_statement.not_kw() {
            self.visit_kw(kw);
        }
        for r#identifier in r#create_extension_statement.r#identifier() {
            self.visit_identifier(r#identifier);
        }
    }
    fn visit_create_function_parameter(
        &mut self,
        r#create_function_parameter: CreateFunctionParameter,
    ) {
        if let Some(kw) = r#create_function_parameter.in_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#create_function_parameter.inout_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#create_function_parameter.out_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#create_function_parameter.variadic_kw() {
            self.visit_kw(kw);
        }
        for r#constrained_type in r#create_function_parameter.r#constrained_type() {
            self.visit_constrained_type(r#constrained_type);
        }
        for r#expression in r#create_function_parameter.r#expression() {
            self.visit_expression(r#expression);
        }
        for r#identifier in r#create_function_parameter.r#identifier() {
            self.visit_identifier(r#identifier);
        }
    }
    fn visit_create_function_parameters(
        &mut self,
        r#create_function_parameters: CreateFunctionParameters,
    ) {
        for r#create_function_parameter in r#create_function_parameters.create_function_parameters()
        {
            self.visit_create_function_parameter(r#create_function_parameter);
        }
    }
    fn visit_create_function_statement(
        &mut self,
        r#create_function_statement: CreateFunctionStatement,
    ) {
        if let Some(kw) = r#create_function_statement.create_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#create_function_statement.function_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#create_function_statement.orreplace_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#create_function_statement.returns_kw() {
            self.visit_kw(kw);
        }
        for r#create_function_parameters in
            r#create_function_statement.r#create_function_parameters()
        {
            self.visit_create_function_parameters(r#create_function_parameters);
        }
        for r#function_body in r#create_function_statement.r#function_body() {
            self.visit_function_body(r#function_body);
        }
        for r#identifier in r#create_function_statement.r#identifier() {
            self.visit_identifier(r#identifier);
        }
        for r#null_hint in r#create_function_statement.r#null_hint() {
            self.visit_null_hint(r#null_hint);
        }
        for r#optimizer_hint in r#create_function_statement.r#optimizer_hint() {
            self.visit_optimizer_hint(r#optimizer_hint);
        }
        for r#parallel_hint in r#create_function_statement.r#parallel_hint() {
            self.visit_parallel_hint(r#parallel_hint);
        }
    }
    fn visit_create_index_statement(&mut self, r#create_index_statement: CreateIndexStatement) {
        if let Some(kw) = r#create_index_statement.create_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#create_index_statement.index_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#create_index_statement.on_kw() {
            self.visit_kw(kw);
        }
        for r#identifier in r#create_index_statement.r#identifier() {
            self.visit_identifier(r#identifier);
        }
        for r#index_table_parameters in r#create_index_statement.r#index_table_parameters() {
            self.visit_index_table_parameters(r#index_table_parameters);
        }
        for r#unique_constraint in r#create_index_statement.r#unique_constraint() {
            self.visit_unique_constraint(r#unique_constraint);
        }
        for r#using_clause in r#create_index_statement.r#using_clause() {
            self.visit_using_clause(r#using_clause);
        }
        for r#where_clause in r#create_index_statement.r#where_clause() {
            self.visit_where_clause(r#where_clause);
        }
    }
    fn visit_create_role_statement(&mut self, r#create_role_statement: CreateRoleStatement) {
        if let Some(kw) = r#create_role_statement.create_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#create_role_statement.role_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#create_role_statement.with_kw() {
            self.visit_kw(kw);
        }
        for r#identifier in r#create_role_statement.r#identifier() {
            self.visit_identifier(r#identifier);
        }
    }
    fn visit_create_schema_statement(&mut self, r#create_schema_statement: CreateSchemaStatement) {
        if let Some(kw) = r#create_schema_statement.create_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#create_schema_statement.exists_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#create_schema_statement.if_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#create_schema_statement.not_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#create_schema_statement.schema_kw() {
            self.visit_kw(kw);
        }
        for r#identifier in r#create_schema_statement.r#identifier() {
            self.visit_identifier(r#identifier);
        }
    }
    fn visit_create_statement(&mut self, r#create_statement: CreateStatement) {
        if let Some(kw) = r#create_statement.create_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#create_statement.temp_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#create_statement.temporary_kw() {
            self.visit_kw(kw);
        }
        for r#sequence in r#create_statement.r#sequence() {
            self.visit_sequence(r#sequence);
        }
    }
    fn visit_create_table_statement(&mut self, r#create_table_statement: CreateTableStatement) {
        if let Some(kw) = r#create_table_statement.create_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#create_table_statement.exists_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#create_table_statement.if_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#create_table_statement.not_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#create_table_statement.table_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#create_table_statement.temporary_kw() {
            self.visit_kw(kw);
        }
        for r#table_parameters in r#create_table_statement.r#table_parameters() {
            self.visit_table_parameters(r#table_parameters);
        }
    }
    fn visit_create_type_statement(&mut self, r#create_type_statement: CreateTypeStatement) {
        if let Some(kw) = r#create_type_statement.as_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#create_type_statement.create_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#create_type_statement.type_kw() {
            self.visit_kw(kw);
        }
        for r#identifier in r#create_type_statement.r#identifier() {
            self.visit_identifier(r#identifier);
        }
        for r#parameters in r#create_type_statement.r#parameters() {
            self.visit_parameters(r#parameters);
        }
    }
    fn visit_direction_constraint(&mut self, r#direction_constraint: DirectionConstraint) {
        if let Some(kw) = r#direction_constraint.asc_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#direction_constraint.desc_kw() {
            self.visit_kw(kw);
        }
    }
    fn visit_distinct_from(&mut self, r#distinct_from: DistinctFrom) {
        if let Some(kw) = r#distinct_from.distinct_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#distinct_from.from_kw() {
            self.visit_kw(kw);
        }
        for r#expression in r#distinct_from.r#expression() {
            self.visit_expression(r#expression);
        }
    }
    fn visit_dotted_name(&mut self, r#dotted_name: DottedName) {
        for r#identifier in r#dotted_name.r#identifier() {
            self.visit_identifier(r#identifier);
        }
    }
    fn visit_drop_statement(&mut self, r#drop_statement: DropStatement) {
        if let Some(kw) = r#drop_statement.drop_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#drop_statement.exists_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#drop_statement.extension_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#drop_statement.if_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#drop_statement.index_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#drop_statement.table_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#drop_statement.tablespace_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#drop_statement.view_kw() {
            self.visit_kw(kw);
        }
    }
    fn visit_exclude_entry(&mut self, r#exclude_entry: ExcludeEntry) {
        if let Some(kw) = r#exclude_entry.with_kw() {
            self.visit_kw(kw);
        }
        for r#binary_operator in r#exclude_entry.r#binary_operator() {
            self.visit_binary_operator(r#binary_operator);
        }
        for r#op_class in r#exclude_entry.r#op_class() {
            self.visit_op_class(r#op_class);
        }
    }
    fn visit_expression(&mut self, r#expression: Expression) {
        for r#false in r#expression.r#false() {
            self.visit_false(r#false);
        }
        for r#null in r#expression.r#null() {
            self.visit_null(r#null);
        }
        for r#true in r#expression.r#true() {
            self.visit_true(r#true);
        }
        for r#argument_reference in r#expression.r#argument_reference() {
            self.visit_argument_reference(r#argument_reference);
        }
        for r#array_element_access in r#expression.r#array_element_access() {
            self.visit_array_element_access(r#array_element_access);
        }
        for r#asterisk_expression in r#expression.r#asterisk_expression() {
            self.visit_asterisk_expression(r#asterisk_expression);
        }
        for r#binary_expression in r#expression.r#binary_expression() {
            self.visit_binary_expression(r#binary_expression);
        }
        for r#boolean_expression in r#expression.r#boolean_expression() {
            self.visit_boolean_expression(r#boolean_expression);
        }
        for r#comparison_operator in r#expression.r#comparison_operator() {
            self.visit_comparison_operator(r#comparison_operator);
        }
        for r#field_access in r#expression.r#field_access() {
            self.visit_field_access(r#field_access);
        }
        for r#function_call in r#expression.r#function_call() {
            self.visit_function_call(r#function_call);
        }
        for r#in_expression in r#expression.r#in_expression() {
            self.visit_in_expression(r#in_expression);
        }
        for r#interval_expression in r#expression.r#interval_expression() {
            self.visit_interval_expression(r#interval_expression);
        }
        for r#is_expression in r#expression.r#is_expression() {
            self.visit_is_expression(r#is_expression);
        }
        for r#number in r#expression.r#number() {
            self.visit_number(r#number);
        }
        for r#parenthesized_expression in r#expression.r#parenthesized_expression() {
            self.visit_parenthesized_expression(r#parenthesized_expression);
        }
        for r#select_subexpression in r#expression.r#select_subexpression() {
            self.visit_select_subexpression(r#select_subexpression);
        }
        for r#string in r#expression.r#string() {
            self.visit_string(r#string);
        }
        for r#type_cast in r#expression.r#type_cast() {
            self.visit_type_cast(r#type_cast);
        }
    }
    fn visit_false(&mut self, r#false: False) {
        if let Some(kw) = r#false.false_kw() {
            self.visit_kw(kw);
        }
    }
    fn visit_field_access(&mut self, r#field_access: FieldAccess) {
        for r#identifier in r#field_access.r#identifier() {
            self.visit_identifier(r#identifier);
        }
        for r#string in r#field_access.r#string() {
            self.visit_string(r#string);
        }
    }
    fn visit_from_clause(&mut self, r#from_clause: FromClause) {
        if let Some(kw) = r#from_clause.from_kw() {
            self.visit_kw(kw);
        }
    }
    fn visit_function_body(&mut self, r#function_body: FunctionBody) {
        if let Some(kw) = r#function_body.as_kw() {
            self.visit_kw(kw);
        }
        for r#select_statement in r#function_body.r#select_statement() {
            self.visit_select_statement(r#select_statement);
        }
    }
    fn visit_function_call(&mut self, r#function_call: FunctionCall) {
        for r#expression in r#function_call.expressions() {
            self.visit_expression(r#expression);
        }
        for r#identifier in r#function_call.r#identifier() {
            self.visit_identifier(r#identifier);
        }
    }
    fn visit_grant_statement(&mut self, r#grant_statement: GrantStatement) {
        if let Some(kw) = r#grant_statement.all_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#grant_statement.database_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#grant_statement.delete_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#grant_statement.grant_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#grant_statement.group_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#grant_statement.insert_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#grant_statement.on_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#grant_statement.option_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#grant_statement.privileges_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#grant_statement.public_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#grant_statement.references_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#grant_statement.schema_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#grant_statement.select_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#grant_statement.sequence_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#grant_statement.table_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#grant_statement.to_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#grant_statement.trigger_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#grant_statement.truncate_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#grant_statement.update_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#grant_statement.usage_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#grant_statement.with_kw() {
            self.visit_kw(kw);
        }
        for r#identifier in r#grant_statement.r#identifier() {
            self.visit_identifier(r#identifier);
        }
    }
    fn visit_group_by_clause(&mut self, r#group_by_clause: GroupByClause) {
        if let Some(kw) = r#group_by_clause.by_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#group_by_clause.group_kw() {
            self.visit_kw(kw);
        }
        for r#group_by_clause_body in r#group_by_clause.r#group_by_clause_body() {
            self.visit_group_by_clause_body(r#group_by_clause_body);
        }
    }
    fn visit_group_by_clause_body(&mut self, r#group_by_clause_body: GroupByClauseBody) {
        for r#expression in r#group_by_clause_body.expressions() {
            self.visit_expression(r#expression);
        }
    }
    fn visit_identifier(&mut self, r#identifier: Identifier) {}
    fn visit_in_expression(&mut self, r#in_expression: InExpression) {
        if let Some(kw) = r#in_expression.in_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#in_expression.not_kw() {
            self.visit_kw(kw);
        }
        for r#expression in r#in_expression.r#expression() {
            self.visit_expression(r#expression);
        }
        for r#tuple in r#in_expression.r#tuple() {
            self.visit_tuple(r#tuple);
        }
    }
    fn visit_index_table_parameters(&mut self, r#index_table_parameters: IndexTableParameters) {
        for r#expression in r#index_table_parameters.r#expression() {
            self.visit_expression(r#expression);
        }
        for r#op_class in r#index_table_parameters.r#op_class() {
            self.visit_op_class(r#op_class);
        }
        for r#ordered_expression in r#index_table_parameters.r#ordered_expression() {
            self.visit_ordered_expression(r#ordered_expression);
        }
    }
    fn visit_initial_mode(&mut self, r#initial_mode: InitialMode) {
        if let Some(kw) = r#initial_mode.deferred_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#initial_mode.immediate_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#initial_mode.initially_kw() {
            self.visit_kw(kw);
        }
    }
    fn visit_insert_statement(&mut self, r#insert_statement: InsertStatement) {
        if let Some(kw) = r#insert_statement.insert_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#insert_statement.into_kw() {
            self.visit_kw(kw);
        }
        for r#values_clause in r#insert_statement.r#values_clause() {
            self.visit_values_clause(r#values_clause);
        }
    }
    fn visit_interval_expression(&mut self, r#interval_expression: IntervalExpression) {
        if let Some(kw) = r#interval_expression.interval_kw() {
            self.visit_kw(kw);
        }
        for r#string in r#interval_expression.r#string() {
            self.visit_string(r#string);
        }
    }
    fn visit_is_expression(&mut self, r#is_expression: IsExpression) {
        for r#false in r#is_expression.r#false() {
            self.visit_false(r#false);
        }
        if let Some(kw) = r#is_expression.is_kw() {
            self.visit_kw(kw);
        }
        for r#null in r#is_expression.r#null() {
            self.visit_null(r#null);
        }
        if let Some(kw) = r#is_expression.not_kw() {
            self.visit_kw(kw);
        }
        for r#true in r#is_expression.r#true() {
            self.visit_true(r#true);
        }
        for r#distinct_from in r#is_expression.r#distinct_from() {
            self.visit_distinct_from(r#distinct_from);
        }
        for r#expression in r#is_expression.r#expression() {
            self.visit_expression(r#expression);
        }
    }
    fn visit_join_clause(&mut self, r#join_clause: JoinClause) {
        if let Some(kw) = r#join_clause.join_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#join_clause.on_kw() {
            self.visit_kw(kw);
        }
        for r#expression in r#join_clause.r#expression() {
            self.visit_expression(r#expression);
        }
        for r#join_type in r#join_clause.r#join_type() {
            self.visit_join_type(r#join_type);
        }
    }
    fn visit_join_type(&mut self, r#join_type: JoinType) {
        if let Some(kw) = r#join_type.full_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#join_type.inner_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#join_type.left_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#join_type.outer_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#join_type.right_kw() {
            self.visit_kw(kw);
        }
    }
    fn visit_mode(&mut self, r#mode: Mode) {
        if let Some(kw) = r#mode.deferrable_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#mode.not_kw() {
            self.visit_kw(kw);
        }
    }
    fn visit_named_constraint(&mut self, r#named_constraint: NamedConstraint) {
        if let Some(kw) = r#named_constraint.constraint_kw() {
            self.visit_kw(kw);
        }
        for r#identifier in r#named_constraint.r#identifier() {
            self.visit_identifier(r#identifier);
        }
    }
    fn visit_null(&mut self, r#null: Null) {
        if let Some(kw) = r#null.null_kw() {
            self.visit_kw(kw);
        }
    }
    fn visit_null_constraint(&mut self, r#null_constraint: NullConstraint) {
        for r#null in r#null_constraint.r#null() {
            self.visit_null(r#null);
        }
        if let Some(kw) = r#null_constraint.not_kw() {
            self.visit_kw(kw);
        }
    }
    fn visit_null_hint(&mut self, r#null_hint: NullHint) {
        if let Some(kw) = r#null_hint.called_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#null_hint.input_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#null_hint.null_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#null_hint.on_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#null_hint.returns_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#null_hint.strict_kw() {
            self.visit_kw(kw);
        }
    }
    fn visit_number(&mut self, r#number: Number) {}
    fn visit_on_delete_action(&mut self, r#on_delete_action: OnDeleteAction) {
        if let Some(kw) = r#on_delete_action.delete_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#on_delete_action.on_kw() {
            self.visit_kw(kw);
        }
    }
    fn visit_on_update_action(&mut self, r#on_update_action: OnUpdateAction) {
        if let Some(kw) = r#on_update_action.on_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#on_update_action.update_kw() {
            self.visit_kw(kw);
        }
    }
    fn visit_op_class(&mut self, r#op_class: OpClass) {}
    fn visit_optimizer_hint(&mut self, r#optimizer_hint: OptimizerHint) {
        if let Some(kw) = r#optimizer_hint.immutable_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#optimizer_hint.stable_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#optimizer_hint.volatile_kw() {
            self.visit_kw(kw);
        }
    }
    fn visit_order_by_clause(&mut self, r#order_by_clause: OrderByClause) {
        if let Some(kw) = r#order_by_clause.by_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#order_by_clause.order_kw() {
            self.visit_kw(kw);
        }
        for r#order_by_clause_body in r#order_by_clause.r#order_by_clause_body() {
            self.visit_order_by_clause_body(r#order_by_clause_body);
        }
    }
    fn visit_order_by_clause_body(&mut self, r#order_by_clause_body: OrderByClauseBody) {
        for r#expression in r#order_by_clause_body.expressions() {
            self.visit_expression(r#expression);
        }
    }
    fn visit_ordered_expression(&mut self, r#ordered_expression: OrderedExpression) {
        if let Some(kw) = r#ordered_expression.asc_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#ordered_expression.desc_kw() {
            self.visit_kw(kw);
        }
        for r#expression in r#ordered_expression.r#expression() {
            self.visit_expression(r#expression);
        }
    }
    fn visit_parallel_hint(&mut self, r#parallel_hint: ParallelHint) {
        if let Some(kw) = r#parallel_hint.parallel_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#parallel_hint.restricted_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#parallel_hint.safe_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#parallel_hint.unsafe_kw() {
            self.visit_kw(kw);
        }
    }
    fn visit_parameter(&mut self, r#parameter: Parameter) {
        for r#constrained_type in r#parameter.r#constrained_type() {
            self.visit_constrained_type(r#constrained_type);
        }
        for r#identifier in r#parameter.r#identifier() {
            self.visit_identifier(r#identifier);
        }
    }
    fn visit_parameters(&mut self, r#parameters: Parameters) {
        for r#parameter in r#parameters.parameters() {
            self.visit_parameter(r#parameter);
        }
    }
    fn visit_parenthesized_expression(
        &mut self,
        r#parenthesized_expression: ParenthesizedExpression,
    ) {
        for r#expression in r#parenthesized_expression.r#expression() {
            self.visit_expression(r#expression);
        }
    }
    fn visit_pg_command(&mut self, r#pg_command: PgCommand) {}
    fn visit_primary_key_constraint(&mut self, r#primary_key_constraint: PrimaryKeyConstraint) {
        if let Some(kw) = r#primary_key_constraint.key_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#primary_key_constraint.primary_kw() {
            self.visit_kw(kw);
        }
    }
    fn visit_references_constraint(&mut self, r#references_constraint: ReferencesConstraint) {
        if let Some(kw) = r#references_constraint.references_kw() {
            self.visit_kw(kw);
        }
        for r#identifier in r#references_constraint.identifiers() {
            self.visit_identifier(r#identifier);
        }
        for r#on_delete_action in r#references_constraint.r#on_delete_action() {
            self.visit_on_delete_action(r#on_delete_action);
        }
        for r#on_update_action in r#references_constraint.r#on_update_action() {
            self.visit_on_update_action(r#on_update_action);
        }
    }
    fn visit_select_clause(&mut self, r#select_clause: SelectClause) {
        if let Some(kw) = r#select_clause.select_kw() {
            self.visit_kw(kw);
        }
        for r#select_clause_body in r#select_clause.r#select_clause_body() {
            self.visit_select_clause_body(r#select_clause_body);
        }
    }
    fn visit_select_clause_body(&mut self, r#select_clause_body: SelectClauseBody) {}
    fn visit_select_statement(&mut self, r#select_statement: SelectStatement) {
        for r#from_clause in r#select_statement.r#from_clause() {
            self.visit_from_clause(r#from_clause);
        }
        for r#group_by_clause in r#select_statement.r#group_by_clause() {
            self.visit_group_by_clause(r#group_by_clause);
        }
        for r#join_clause in r#select_statement.join_clauses() {
            self.visit_join_clause(r#join_clause);
        }
        for r#order_by_clause in r#select_statement.r#order_by_clause() {
            self.visit_order_by_clause(r#order_by_clause);
        }
        for r#select_clause in r#select_statement.r#select_clause() {
            self.visit_select_clause(r#select_clause);
        }
        for r#where_clause in r#select_statement.r#where_clause() {
            self.visit_where_clause(r#where_clause);
        }
    }
    fn visit_select_subexpression(&mut self, r#select_subexpression: SelectSubexpression) {
        for r#select_statement in r#select_subexpression.r#select_statement() {
            self.visit_select_statement(r#select_statement);
        }
    }
    fn visit_sequence(&mut self, r#sequence: Sequence) {
        if let Some(kw) = r#sequence.as_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#sequence.by_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#sequence.cache_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#sequence.exists_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#sequence.if_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#sequence.increment_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#sequence.maxvalue_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#sequence.minvalue_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#sequence.no_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#sequence.not_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#sequence.owned_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#sequence.sequence_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#sequence.start_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#sequence.with_kw() {
            self.visit_kw(kw);
        }
        for r#dotted_name in r#sequence.r#dotted_name() {
            self.visit_dotted_name(r#dotted_name);
        }
        for r#number in r#sequence.r#number() {
            self.visit_number(r#number);
        }
        for r#type in r#sequence.r#type() {
            self.visit_type(r#type);
        }
    }
    fn visit_set_clause(&mut self, r#set_clause: SetClause) {
        if let Some(kw) = r#set_clause.set_kw() {
            self.visit_kw(kw);
        }
        for r#set_clause_body in r#set_clause.r#set_clause_body() {
            self.visit_set_clause_body(r#set_clause_body);
        }
    }
    fn visit_set_clause_body(&mut self, r#set_clause_body: SetClauseBody) {
        for r#assigment_expression in r#set_clause_body.assigment_expressions() {
            self.visit_assigment_expression(r#assigment_expression);
        }
    }
    fn visit_set_statement(&mut self, r#set_statement: SetStatement) {
        if let Some(kw) = r#set_statement.default_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#set_statement.local_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#set_statement.session_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#set_statement.set_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#set_statement.to_kw() {
            self.visit_kw(kw);
        }
        for r#expression in r#set_statement.r#expression() {
            self.visit_expression(r#expression);
        }
        for r#identifier in r#set_statement.r#identifier() {
            self.visit_identifier(r#identifier);
        }
    }
    fn visit_setof(&mut self, r#setof: Setof) {
        if let Some(kw) = r#setof.setof_kw() {
            self.visit_kw(kw);
        }
        for r#constrained_type in r#setof.r#constrained_type() {
            self.visit_constrained_type(r#constrained_type);
        }
    }
    fn visit_source_file(&mut self, r#source_file: SourceFile) {
        for r#statement in r#source_file.statements() {
            self.visit_statement(r#statement);
        }
    }
    fn visit_statement(&mut self, r#statement: Statement) {
        for r#alter_statement in r#statement.r#alter_statement() {
            self.visit_alter_statement(r#alter_statement);
        }
        for r#create_domain_statement in r#statement.r#create_domain_statement() {
            self.visit_create_domain_statement(r#create_domain_statement);
        }
        for r#create_extension_statement in r#statement.r#create_extension_statement() {
            self.visit_create_extension_statement(r#create_extension_statement);
        }
        for r#create_function_statement in r#statement.r#create_function_statement() {
            self.visit_create_function_statement(r#create_function_statement);
        }
        for r#create_index_statement in r#statement.r#create_index_statement() {
            self.visit_create_index_statement(r#create_index_statement);
        }
        for r#create_role_statement in r#statement.r#create_role_statement() {
            self.visit_create_role_statement(r#create_role_statement);
        }
        for r#create_schema_statement in r#statement.r#create_schema_statement() {
            self.visit_create_schema_statement(r#create_schema_statement);
        }
        for r#create_statement in r#statement.r#create_statement() {
            self.visit_create_statement(r#create_statement);
        }
        for r#create_table_statement in r#statement.r#create_table_statement() {
            self.visit_create_table_statement(r#create_table_statement);
        }
        for r#create_type_statement in r#statement.r#create_type_statement() {
            self.visit_create_type_statement(r#create_type_statement);
        }
        for r#drop_statement in r#statement.r#drop_statement() {
            self.visit_drop_statement(r#drop_statement);
        }
        for r#grant_statement in r#statement.r#grant_statement() {
            self.visit_grant_statement(r#grant_statement);
        }
        for r#insert_statement in r#statement.r#insert_statement() {
            self.visit_insert_statement(r#insert_statement);
        }
        for r#pg_command in r#statement.r#pg_command() {
            self.visit_pg_command(r#pg_command);
        }
        for r#select_statement in r#statement.r#select_statement() {
            self.visit_select_statement(r#select_statement);
        }
        for r#set_statement in r#statement.r#set_statement() {
            self.visit_set_statement(r#set_statement);
        }
        for r#update_statement in r#statement.r#update_statement() {
            self.visit_update_statement(r#update_statement);
        }
    }
    fn visit_string(&mut self, r#string: String) {}
    fn visit_table_column(&mut self, r#table_column: TableColumn) {
        for r#auto_increment_constraint in r#table_column.r#auto_increment_constraint() {
            self.visit_auto_increment_constraint(r#auto_increment_constraint);
        }
        for r#check_constraint in r#table_column.r#check_constraint() {
            self.visit_check_constraint(r#check_constraint);
        }
        for r#column_default in r#table_column.r#column_default() {
            self.visit_column_default(r#column_default);
        }
        for r#direction_constraint in r#table_column.r#direction_constraint() {
            self.visit_direction_constraint(r#direction_constraint);
        }
        for r#named_constraint in r#table_column.r#named_constraint() {
            self.visit_named_constraint(r#named_constraint);
        }
        for r#null_constraint in r#table_column.r#null_constraint() {
            self.visit_null_constraint(r#null_constraint);
        }
        for r#primary_key_constraint in r#table_column.r#primary_key_constraint() {
            self.visit_primary_key_constraint(r#primary_key_constraint);
        }
        for r#references_constraint in r#table_column.r#references_constraint() {
            self.visit_references_constraint(r#references_constraint);
        }
        for r#time_zone_constraint in r#table_column.r#time_zone_constraint() {
            self.visit_time_zone_constraint(r#time_zone_constraint);
        }
        for r#unique_constraint in r#table_column.r#unique_constraint() {
            self.visit_unique_constraint(r#unique_constraint);
        }
    }
    fn visit_table_constraint_check(&mut self, r#table_constraint_check: TableConstraintCheck) {
        if let Some(kw) = r#table_constraint_check.check_kw() {
            self.visit_kw(kw);
        }
        for r#expression in r#table_constraint_check.r#expression() {
            self.visit_expression(r#expression);
        }
    }
    fn visit_table_constraint_exclude(
        &mut self,
        r#table_constraint_exclude: TableConstraintExclude,
    ) {
        if let Some(kw) = r#table_constraint_exclude.exclude_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#table_constraint_exclude.using_kw() {
            self.visit_kw(kw);
        }
        for r#exclude_entry in r#table_constraint_exclude.exclude_entrys() {
            self.visit_exclude_entry(r#exclude_entry);
        }
    }
    fn visit_table_constraint_foreign_key(
        &mut self,
        r#table_constraint_foreign_key: TableConstraintForeignKey,
    ) {
        if let Some(kw) = r#table_constraint_foreign_key.foreign_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#table_constraint_foreign_key.key_kw() {
            self.visit_kw(kw);
        }
        for r#identifier in r#table_constraint_foreign_key.identifiers() {
            self.visit_identifier(r#identifier);
        }
        for r#references_constraint in r#table_constraint_foreign_key.r#references_constraint() {
            self.visit_references_constraint(r#references_constraint);
        }
    }
    fn visit_table_constraint_primary_key(
        &mut self,
        r#table_constraint_primary_key: TableConstraintPrimaryKey,
    ) {
        if let Some(kw) = r#table_constraint_primary_key.key_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#table_constraint_primary_key.primary_kw() {
            self.visit_kw(kw);
        }
    }
    fn visit_table_constraint_unique(&mut self, r#table_constraint_unique: TableConstraintUnique) {
        if let Some(kw) = r#table_constraint_unique.unique_kw() {
            self.visit_kw(kw);
        }
    }
    fn visit_table_parameters(&mut self, r#table_parameters: TableParameters) {
        for r#table_column in r#table_parameters.r#table_column() {
            self.visit_table_column(r#table_column);
        }
    }
    fn visit_time_zone_constraint(&mut self, r#time_zone_constraint: TimeZoneConstraint) {
        if let Some(kw) = r#time_zone_constraint.time_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#time_zone_constraint.with_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#time_zone_constraint.without_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#time_zone_constraint.zone_kw() {
            self.visit_kw(kw);
        }
    }
    fn visit_true(&mut self, r#true: True) {
        if let Some(kw) = r#true.true_kw() {
            self.visit_kw(kw);
        }
    }
    fn visit_tuple(&mut self, r#tuple: Tuple) {
        for r#expression in r#tuple.expressions() {
            self.visit_expression(r#expression);
        }
    }
    fn visit_type(&mut self, r#type: Type) {
        for r#number in r#type.r#number() {
            self.visit_number(r#number);
        }
    }
    fn visit_type_cast(&mut self, r#type_cast: TypeCast) {
        for r#function_call in r#type_cast.r#function_call() {
            self.visit_function_call(r#function_call);
        }
        for r#identifier in r#type_cast.r#identifier() {
            self.visit_identifier(r#identifier);
        }
        for r#parenthesized_expression in r#type_cast.r#parenthesized_expression() {
            self.visit_parenthesized_expression(r#parenthesized_expression);
        }
        for r#string in r#type_cast.r#string() {
            self.visit_string(r#string);
        }
    }
    fn visit_unique_constraint(&mut self, r#unique_constraint: UniqueConstraint) {
        if let Some(kw) = r#unique_constraint.unique_kw() {
            self.visit_kw(kw);
        }
    }
    fn visit_update_statement(&mut self, r#update_statement: UpdateStatement) {
        if let Some(kw) = r#update_statement.update_kw() {
            self.visit_kw(kw);
        }
        for r#identifier in r#update_statement.r#identifier() {
            self.visit_identifier(r#identifier);
        }
        for r#set_clause in r#update_statement.r#set_clause() {
            self.visit_set_clause(r#set_clause);
        }
        for r#where_clause in r#update_statement.r#where_clause() {
            self.visit_where_clause(r#where_clause);
        }
    }
    fn visit_using_clause(&mut self, r#using_clause: UsingClause) {
        if let Some(kw) = r#using_clause.using_kw() {
            self.visit_kw(kw);
        }
        for r#identifier in r#using_clause.r#identifier() {
            self.visit_identifier(r#identifier);
        }
    }
    fn visit_values_clause(&mut self, r#values_clause: ValuesClause) {
        if let Some(kw) = r#values_clause.values_kw() {
            self.visit_kw(kw);
        }
        for r#values_clause_body in r#values_clause.r#values_clause_body() {
            self.visit_values_clause_body(r#values_clause_body);
        }
    }
    fn visit_values_clause_body(&mut self, r#values_clause_body: ValuesClauseBody) {
        for r#expression in r#values_clause_body.expressions() {
            self.visit_expression(r#expression);
        }
    }
    fn visit_where_clause(&mut self, r#where_clause: WhereClause) {
        if let Some(kw) = r#where_clause.where_kw() {
            self.visit_kw(kw);
        }
        for r#expression in r#where_clause.r#expression() {
            self.visit_expression(r#expression);
        }
    }
}
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum SyntaxKind {
    False,
    Null,
    True,
    AddKw,
    AllKw,
    AlterKw,
    AlterStatement,
    AlterTable,
    AlterTableAction,
    AlterTableActionAdd,
    AlterTableActionAlterColumn,
    AndKw,
    ArgumentReference,
    ArrayElementAccess,
    ArrayType,
    AsKw,
    AscKw,
    AssigmentExpression,
    AsteriskExpression,
    AutoIncrementConstraint,
    AutoincrementKw,
    BinaryExpression,
    BinaryOperator,
    BooleanExpression,
    ByKw,
    CacheKw,
    CalledKw,
    CheckKw,
    CheckConstraint,
    ColumnKw,
    ColumnDefault,
    Comment,
    ComparisonOperator,
    ConstrainedType,
    ConstraintKw,
    CreateKw,
    CreateDomainStatement,
    CreateExtensionStatement,
    CreateFunctionParameter,
    CreateFunctionParameters,
    CreateFunctionStatement,
    CreateIndexStatement,
    CreateRoleStatement,
    CreateSchemaStatement,
    CreateStatement,
    CreateTableStatement,
    CreateTypeStatement,
    DatabaseKw,
    DefaultKw,
    DeferrableKw,
    DeferredKw,
    DeleteKw,
    DescKw,
    DirectionConstraint,
    DistinctKw,
    DistinctFrom,
    DomainKw,
    DottedName,
    DropKw,
    DropStatement,
    ExcludeKw,
    ExcludeEntry,
    ExistsKw,
    Expression,
    ExtensionKw,
    FalseKw,
    FieldAccess,
    ForeignKw,
    FromKw,
    FromClause,
    FullKw,
    FunctionKw,
    FunctionBody,
    FunctionCall,
    GrantKw,
    GrantStatement,
    GroupKw,
    GroupByClause,
    GroupByClauseBody,
    Identifier,
    IfKw,
    ImmediateKw,
    ImmutableKw,
    InKw,
    InExpression,
    IncrementKw,
    IndexKw,
    IndexTableParameters,
    InitialMode,
    InitiallyKw,
    InnerKw,
    InoutKw,
    InputKw,
    InsertKw,
    InsertStatement,
    IntervalKw,
    IntervalExpression,
    IntoKw,
    IsKw,
    IsExpression,
    JoinKw,
    JoinClause,
    JoinType,
    KeyKw,
    LeftKw,
    LocalKw,
    MaxvalueKw,
    MinvalueKw,
    Mode,
    NamedConstraint,
    NoKw,
    NotKw,
    NullKw,
    NullConstraint,
    NullHint,
    Number,
    OnKw,
    OnDeleteAction,
    OnUpdateAction,
    OnlyKw,
    OpClass,
    OptimizerHint,
    OptionKw,
    OrKw,
    OrderKw,
    OrderByClause,
    OrderByClauseBody,
    OrderedExpression,
    OrreplaceKw,
    OutKw,
    OuterKw,
    OwnedKw,
    ParallelKw,
    ParallelHint,
    Parameter,
    Parameters,
    ParenthesizedExpression,
    PgCommand,
    PrimaryKw,
    PrimaryKeyConstraint,
    PrivilegesKw,
    PublicKw,
    ReferencesKw,
    ReferencesConstraint,
    RestrictedKw,
    ReturnsKw,
    RightKw,
    RoleKw,
    SafeKw,
    SchemaKw,
    SelectKw,
    SelectClause,
    SelectClauseBody,
    SelectStatement,
    SelectSubexpression,
    Sequence,
    SequenceKw,
    SessionKw,
    SetKw,
    SetClause,
    SetClauseBody,
    SetStatement,
    Setof,
    SetofKw,
    SourceFile,
    StableKw,
    StartKw,
    Statement,
    StrictKw,
    String,
    TableKw,
    TableColumn,
    TableConstraintCheck,
    TableConstraintExclude,
    TableConstraintForeignKey,
    TableConstraintPrimaryKey,
    TableConstraintUnique,
    TableParameters,
    TablespaceKw,
    TempKw,
    TemporaryKw,
    TimeKw,
    TimeZoneConstraint,
    ToKw,
    TriggerKw,
    TrueKw,
    TruncateKw,
    Tuple,
    Type,
    TypeKw,
    TypeCast,
    UniqueKw,
    UniqueConstraint,
    UnsafeKw,
    UpdateKw,
    UpdateStatement,
    UsageKw,
    UsingKw,
    UsingClause,
    ValuesKw,
    ValuesClause,
    ValuesClauseBody,
    VariadicKw,
    ViewKw,
    VolatileKw,
    WhereKw,
    WhereClause,
    WithKw,
    WithoutKw,
    ZoneKw,
    Token,
    Err,
}
impl TryFrom<&'static str> for SyntaxKind {
    type Error = ();
    fn try_from(s: &'static str) -> Result<Self, Self::Error> {
        match s {
            "false" => Ok(Self::False),
            "null" => Ok(Self::Null),
            "true" => Ok(Self::True),
            "ADD" => Ok(Self::AddKw),
            "ALL" => Ok(Self::AllKw),
            "ALTER" => Ok(Self::AlterKw),
            "alter_statement" => Ok(Self::AlterStatement),
            "alter_table" => Ok(Self::AlterTable),
            "alter_table_action" => Ok(Self::AlterTableAction),
            "alter_table_action_add" => Ok(Self::AlterTableActionAdd),
            "alter_table_action_alter_column" => Ok(Self::AlterTableActionAlterColumn),
            "AND" => Ok(Self::AndKw),
            "argument_reference" => Ok(Self::ArgumentReference),
            "array_element_access" => Ok(Self::ArrayElementAccess),
            "array_type" => Ok(Self::ArrayType),
            "AS" => Ok(Self::AsKw),
            "ASC" => Ok(Self::AscKw),
            "assigment_expression" => Ok(Self::AssigmentExpression),
            "asterisk_expression" => Ok(Self::AsteriskExpression),
            "auto_increment_constraint" => Ok(Self::AutoIncrementConstraint),
            "AUTOINCREMENT" => Ok(Self::AutoincrementKw),
            "binary_expression" => Ok(Self::BinaryExpression),
            "binary_operator" => Ok(Self::BinaryOperator),
            "boolean_expression" => Ok(Self::BooleanExpression),
            "BY" => Ok(Self::ByKw),
            "CACHE" => Ok(Self::CacheKw),
            "CALLED" => Ok(Self::CalledKw),
            "CHECK" => Ok(Self::CheckKw),
            "check_constraint" => Ok(Self::CheckConstraint),
            "COLUMN" => Ok(Self::ColumnKw),
            "column_default" => Ok(Self::ColumnDefault),
            "comment" => Ok(Self::Comment),
            "comparison_operator" => Ok(Self::ComparisonOperator),
            "constrained_type" => Ok(Self::ConstrainedType),
            "CONSTRAINT" => Ok(Self::ConstraintKw),
            "CREATE" => Ok(Self::CreateKw),
            "create_domain_statement" => Ok(Self::CreateDomainStatement),
            "create_extension_statement" => Ok(Self::CreateExtensionStatement),
            "create_function_parameter" => Ok(Self::CreateFunctionParameter),
            "create_function_parameters" => Ok(Self::CreateFunctionParameters),
            "create_function_statement" => Ok(Self::CreateFunctionStatement),
            "create_index_statement" => Ok(Self::CreateIndexStatement),
            "create_role_statement" => Ok(Self::CreateRoleStatement),
            "create_schema_statement" => Ok(Self::CreateSchemaStatement),
            "create_statement" => Ok(Self::CreateStatement),
            "create_table_statement" => Ok(Self::CreateTableStatement),
            "create_type_statement" => Ok(Self::CreateTypeStatement),
            "DATABASE" => Ok(Self::DatabaseKw),
            "DEFAULT" => Ok(Self::DefaultKw),
            "DEFERRABLE" => Ok(Self::DeferrableKw),
            "DEFERRED" => Ok(Self::DeferredKw),
            "DELETE" => Ok(Self::DeleteKw),
            "DESC" => Ok(Self::DescKw),
            "direction_constraint" => Ok(Self::DirectionConstraint),
            "DISTINCT" => Ok(Self::DistinctKw),
            "distinct_from" => Ok(Self::DistinctFrom),
            "DOMAIN" => Ok(Self::DomainKw),
            "dotted_name" => Ok(Self::DottedName),
            "DROP" => Ok(Self::DropKw),
            "drop_statement" => Ok(Self::DropStatement),
            "EXCLUDE" => Ok(Self::ExcludeKw),
            "exclude_entry" => Ok(Self::ExcludeEntry),
            "EXISTS" => Ok(Self::ExistsKw),
            "expression" => Ok(Self::Expression),
            "EXTENSION" => Ok(Self::ExtensionKw),
            "FALSE" => Ok(Self::FalseKw),
            "field_access" => Ok(Self::FieldAccess),
            "FOREIGN" => Ok(Self::ForeignKw),
            "FROM" => Ok(Self::FromKw),
            "from_clause" => Ok(Self::FromClause),
            "FULL" => Ok(Self::FullKw),
            "FUNCTION" => Ok(Self::FunctionKw),
            "function_body" => Ok(Self::FunctionBody),
            "function_call" => Ok(Self::FunctionCall),
            "GRANT" => Ok(Self::GrantKw),
            "grant_statement" => Ok(Self::GrantStatement),
            "GROUP" => Ok(Self::GroupKw),
            "group_by_clause" => Ok(Self::GroupByClause),
            "group_by_clause_body" => Ok(Self::GroupByClauseBody),
            "identifier" => Ok(Self::Identifier),
            "IF" => Ok(Self::IfKw),
            "IMMEDIATE" => Ok(Self::ImmediateKw),
            "IMMUTABLE" => Ok(Self::ImmutableKw),
            "IN" => Ok(Self::InKw),
            "in_expression" => Ok(Self::InExpression),
            "INCREMENT" => Ok(Self::IncrementKw),
            "INDEX" => Ok(Self::IndexKw),
            "index_table_parameters" => Ok(Self::IndexTableParameters),
            "initial_mode" => Ok(Self::InitialMode),
            "INITIALLY" => Ok(Self::InitiallyKw),
            "INNER" => Ok(Self::InnerKw),
            "INOUT" => Ok(Self::InoutKw),
            "INPUT" => Ok(Self::InputKw),
            "INSERT" => Ok(Self::InsertKw),
            "insert_statement" => Ok(Self::InsertStatement),
            "INTERVAL" => Ok(Self::IntervalKw),
            "interval_expression" => Ok(Self::IntervalExpression),
            "INTO" => Ok(Self::IntoKw),
            "IS" => Ok(Self::IsKw),
            "is_expression" => Ok(Self::IsExpression),
            "JOIN" => Ok(Self::JoinKw),
            "join_clause" => Ok(Self::JoinClause),
            "join_type" => Ok(Self::JoinType),
            "KEY" => Ok(Self::KeyKw),
            "LEFT" => Ok(Self::LeftKw),
            "LOCAL" => Ok(Self::LocalKw),
            "MAXVALUE" => Ok(Self::MaxvalueKw),
            "MINVALUE" => Ok(Self::MinvalueKw),
            "mode" => Ok(Self::Mode),
            "named_constraint" => Ok(Self::NamedConstraint),
            "NO" => Ok(Self::NoKw),
            "NOT" => Ok(Self::NotKw),
            "NULL" => Ok(Self::NullKw),
            "null_constraint" => Ok(Self::NullConstraint),
            "null_hint" => Ok(Self::NullHint),
            "number" => Ok(Self::Number),
            "ON" => Ok(Self::OnKw),
            "on_delete_action" => Ok(Self::OnDeleteAction),
            "on_update_action" => Ok(Self::OnUpdateAction),
            "ONLY" => Ok(Self::OnlyKw),
            "op_class" => Ok(Self::OpClass),
            "optimizer_hint" => Ok(Self::OptimizerHint),
            "OPTION" => Ok(Self::OptionKw),
            "OR" => Ok(Self::OrKw),
            "ORDER" => Ok(Self::OrderKw),
            "order_by_clause" => Ok(Self::OrderByClause),
            "order_by_clause_body" => Ok(Self::OrderByClauseBody),
            "ordered_expression" => Ok(Self::OrderedExpression),
            "ORREPLACE" => Ok(Self::OrreplaceKw),
            "OUT" => Ok(Self::OutKw),
            "OUTER" => Ok(Self::OuterKw),
            "OWNED" => Ok(Self::OwnedKw),
            "PARALLEL" => Ok(Self::ParallelKw),
            "parallel_hint" => Ok(Self::ParallelHint),
            "parameter" => Ok(Self::Parameter),
            "parameters" => Ok(Self::Parameters),
            "parenthesized_expression" => Ok(Self::ParenthesizedExpression),
            "pg_command" => Ok(Self::PgCommand),
            "PRIMARY" => Ok(Self::PrimaryKw),
            "primary_key_constraint" => Ok(Self::PrimaryKeyConstraint),
            "PRIVILEGES" => Ok(Self::PrivilegesKw),
            "PUBLIC" => Ok(Self::PublicKw),
            "REFERENCES" => Ok(Self::ReferencesKw),
            "references_constraint" => Ok(Self::ReferencesConstraint),
            "RESTRICTED" => Ok(Self::RestrictedKw),
            "RETURNS" => Ok(Self::ReturnsKw),
            "RIGHT" => Ok(Self::RightKw),
            "ROLE" => Ok(Self::RoleKw),
            "SAFE" => Ok(Self::SafeKw),
            "SCHEMA" => Ok(Self::SchemaKw),
            "SELECT" => Ok(Self::SelectKw),
            "select_clause" => Ok(Self::SelectClause),
            "select_clause_body" => Ok(Self::SelectClauseBody),
            "select_statement" => Ok(Self::SelectStatement),
            "select_subexpression" => Ok(Self::SelectSubexpression),
            "sequence" => Ok(Self::Sequence),
            "SEQUENCE" => Ok(Self::SequenceKw),
            "SESSION" => Ok(Self::SessionKw),
            "SET" => Ok(Self::SetKw),
            "set_clause" => Ok(Self::SetClause),
            "set_clause_body" => Ok(Self::SetClauseBody),
            "set_statement" => Ok(Self::SetStatement),
            "setof" => Ok(Self::Setof),
            "SETOF" => Ok(Self::SetofKw),
            "source_file" => Ok(Self::SourceFile),
            "STABLE" => Ok(Self::StableKw),
            "START" => Ok(Self::StartKw),
            "statement" => Ok(Self::Statement),
            "STRICT" => Ok(Self::StrictKw),
            "string" => Ok(Self::String),
            "TABLE" => Ok(Self::TableKw),
            "table_column" => Ok(Self::TableColumn),
            "table_constraint_check" => Ok(Self::TableConstraintCheck),
            "table_constraint_exclude" => Ok(Self::TableConstraintExclude),
            "table_constraint_foreign_key" => Ok(Self::TableConstraintForeignKey),
            "table_constraint_primary_key" => Ok(Self::TableConstraintPrimaryKey),
            "table_constraint_unique" => Ok(Self::TableConstraintUnique),
            "table_parameters" => Ok(Self::TableParameters),
            "TABLESPACE" => Ok(Self::TablespaceKw),
            "TEMP" => Ok(Self::TempKw),
            "TEMPORARY" => Ok(Self::TemporaryKw),
            "TIME" => Ok(Self::TimeKw),
            "time_zone_constraint" => Ok(Self::TimeZoneConstraint),
            "TO" => Ok(Self::ToKw),
            "TRIGGER" => Ok(Self::TriggerKw),
            "TRUE" => Ok(Self::TrueKw),
            "TRUNCATE" => Ok(Self::TruncateKw),
            "tuple" => Ok(Self::Tuple),
            "type" => Ok(Self::Type),
            "TYPE" => Ok(Self::TypeKw),
            "type_cast" => Ok(Self::TypeCast),
            "UNIQUE" => Ok(Self::UniqueKw),
            "unique_constraint" => Ok(Self::UniqueConstraint),
            "UNSAFE" => Ok(Self::UnsafeKw),
            "UPDATE" => Ok(Self::UpdateKw),
            "update_statement" => Ok(Self::UpdateStatement),
            "USAGE" => Ok(Self::UsageKw),
            "USING" => Ok(Self::UsingKw),
            "using_clause" => Ok(Self::UsingClause),
            "VALUES" => Ok(Self::ValuesKw),
            "values_clause" => Ok(Self::ValuesClause),
            "values_clause_body" => Ok(Self::ValuesClauseBody),
            "VARIADIC" => Ok(Self::VariadicKw),
            "VIEW" => Ok(Self::ViewKw),
            "VOLATILE" => Ok(Self::VolatileKw),
            "WHERE" => Ok(Self::WhereKw),
            "where_clause" => Ok(Self::WhereClause),
            "WITH" => Ok(Self::WithKw),
            "WITHOUT" => Ok(Self::WithoutKw),
            "ZONE" => Ok(Self::ZoneKw),
            "ERROR" => Ok(Self::Err),
            _ => Err(()),
        }
    }
}
