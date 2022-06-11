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
pub struct False(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Null(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct True(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterStatement(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterTable(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterTableAction(SyntaxNode);
impl Node for AlterTableAction {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::AlterTableAction
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self(syntax))
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterTableActionAdd(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterTableActionAlterColumn(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArgumentReference(SyntaxNode);
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
pub struct ArrayElementAccess(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayType(SyntaxNode);
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
pub struct AssigmentExpression(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AsteriskExpression(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AutoIncrementConstraint(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinaryExpression(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinaryOperator(SyntaxNode);
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
pub struct BooleanExpression(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CheckConstraint(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ColumnDefault(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Comment(SyntaxNode);
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
pub struct ComparisonOperator(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstrainedType(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateDomainStatement(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateExtensionStatement(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateFunctionParameter(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateFunctionParameters(SyntaxNode);
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
pub struct CreateFunctionStatement(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateIndexStatement(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateRoleStatement(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateSchemaStatement(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateStatement(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateTableStatement(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateTypeStatement(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DirectionConstraint(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DistinctFrom(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DottedName(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropStatement(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExcludeEntry(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldAccess(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FromClause(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionBody(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionCall(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GrantStatement(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GroupByClause(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GroupByClauseBody(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(SyntaxNode);
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
pub struct InExpression(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndexTableParameters(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InitialMode(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InsertStatement(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntervalExpression(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IsExpression(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JoinClause(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JoinType(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Mode(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamedConstraint(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NullConstraint(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NullHint(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Number(SyntaxNode);
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
pub struct OnDeleteAction(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OnUpdateAction(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OpClass(SyntaxNode);
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
pub struct OptimizerHint(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderByClause(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderByClauseBody(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderedExpression(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParallelHint(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Parameter(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Parameters(SyntaxNode);
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
pub struct PgCommand(SyntaxNode);
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
pub struct PrimaryKeyConstraint(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReferencesConstraint(SyntaxNode);
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
    pub fn identifiers(&self) -> impl Iterator<Item = Identifier> {
        self.children()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SelectClause(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SelectClauseBody(SyntaxNode);
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
pub struct SelectStatement(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SelectSubexpression(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sequence(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetClause(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetClauseBody(SyntaxNode);
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
pub struct SetStatement(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Setof(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceFile(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct String(SyntaxNode);
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
pub struct TableColumn(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableConstraintCheck(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableConstraintExclude(SyntaxNode);
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
    pub fn exclude_entrys(&self) -> impl Iterator<Item = ExcludeEntry> {
        self.children()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableConstraintForeignKey(SyntaxNode);
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
    pub fn identifiers(&self) -> impl Iterator<Item = Identifier> {
        self.children()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableConstraintPrimaryKey(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableConstraintUnique(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableParameters(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TimeZoneConstraint(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tuple(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Type(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeCast(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UniqueConstraint(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UpdateStatement(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UsingClause(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValuesClause(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValuesClauseBody(SyntaxNode);
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhereClause(SyntaxNode);
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
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum SyntaxKind {
    False,
    Null,
    True,
    AlterStatement,
    AlterTable,
    AlterTableAction,
    AlterTableActionAdd,
    AlterTableActionAlterColumn,
    ArgumentReference,
    ArrayElementAccess,
    ArrayType,
    AssigmentExpression,
    AsteriskExpression,
    AutoIncrementConstraint,
    BinaryExpression,
    BinaryOperator,
    BooleanExpression,
    CheckConstraint,
    ColumnDefault,
    Comment,
    ComparisonOperator,
    ConstrainedType,
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
    DirectionConstraint,
    DistinctFrom,
    DottedName,
    DropStatement,
    ExcludeEntry,
    FieldAccess,
    FromClause,
    FunctionBody,
    FunctionCall,
    GrantStatement,
    GroupByClause,
    GroupByClauseBody,
    Identifier,
    InExpression,
    IndexTableParameters,
    InitialMode,
    InsertStatement,
    IntervalExpression,
    IsExpression,
    JoinClause,
    JoinType,
    Mode,
    NamedConstraint,
    NullConstraint,
    NullHint,
    Number,
    OnDeleteAction,
    OnUpdateAction,
    OpClass,
    OptimizerHint,
    OrderByClause,
    OrderByClauseBody,
    OrderedExpression,
    ParallelHint,
    Parameter,
    Parameters,
    PgCommand,
    PrimaryKeyConstraint,
    ReferencesConstraint,
    SelectClause,
    SelectClauseBody,
    SelectStatement,
    SelectSubexpression,
    Sequence,
    SetClause,
    SetClauseBody,
    SetStatement,
    Setof,
    SourceFile,
    String,
    TableColumn,
    TableConstraintCheck,
    TableConstraintExclude,
    TableConstraintForeignKey,
    TableConstraintPrimaryKey,
    TableConstraintUnique,
    TableParameters,
    TimeZoneConstraint,
    Tuple,
    Type,
    TypeCast,
    UniqueConstraint,
    UpdateStatement,
    UsingClause,
    ValuesClause,
    ValuesClauseBody,
    WhereClause,
}
impl From<&'static str> for SyntaxKind {
    fn from(s: &'static str) -> Self {
        match s {
            "FALSE" => Self::False,
            "NULL" => Self::Null,
            "TRUE" => Self::True,
            "alter_statement" => Self::AlterStatement,
            "alter_table" => Self::AlterTable,
            "alter_table_action" => Self::AlterTableAction,
            "alter_table_action_add" => Self::AlterTableActionAdd,
            "alter_table_action_alter_column" => Self::AlterTableActionAlterColumn,
            "argument_reference" => Self::ArgumentReference,
            "array_element_access" => Self::ArrayElementAccess,
            "array_type" => Self::ArrayType,
            "assigment_expression" => Self::AssigmentExpression,
            "asterisk_expression" => Self::AsteriskExpression,
            "auto_increment_constraint" => Self::AutoIncrementConstraint,
            "binary_expression" => Self::BinaryExpression,
            "binary_operator" => Self::BinaryOperator,
            "boolean_expression" => Self::BooleanExpression,
            "check_constraint" => Self::CheckConstraint,
            "column_default" => Self::ColumnDefault,
            "comment" => Self::Comment,
            "comparison_operator" => Self::ComparisonOperator,
            "constrained_type" => Self::ConstrainedType,
            "create_domain_statement" => Self::CreateDomainStatement,
            "create_extension_statement" => Self::CreateExtensionStatement,
            "create_function_parameter" => Self::CreateFunctionParameter,
            "create_function_parameters" => Self::CreateFunctionParameters,
            "create_function_statement" => Self::CreateFunctionStatement,
            "create_index_statement" => Self::CreateIndexStatement,
            "create_role_statement" => Self::CreateRoleStatement,
            "create_schema_statement" => Self::CreateSchemaStatement,
            "create_statement" => Self::CreateStatement,
            "create_table_statement" => Self::CreateTableStatement,
            "create_type_statement" => Self::CreateTypeStatement,
            "direction_constraint" => Self::DirectionConstraint,
            "distinct_from" => Self::DistinctFrom,
            "dotted_name" => Self::DottedName,
            "drop_statement" => Self::DropStatement,
            "exclude_entry" => Self::ExcludeEntry,
            "field_access" => Self::FieldAccess,
            "from_clause" => Self::FromClause,
            "function_body" => Self::FunctionBody,
            "function_call" => Self::FunctionCall,
            "grant_statement" => Self::GrantStatement,
            "group_by_clause" => Self::GroupByClause,
            "group_by_clause_body" => Self::GroupByClauseBody,
            "identifier" => Self::Identifier,
            "in_expression" => Self::InExpression,
            "index_table_parameters" => Self::IndexTableParameters,
            "initial_mode" => Self::InitialMode,
            "insert_statement" => Self::InsertStatement,
            "interval_expression" => Self::IntervalExpression,
            "is_expression" => Self::IsExpression,
            "join_clause" => Self::JoinClause,
            "join_type" => Self::JoinType,
            "mode" => Self::Mode,
            "named_constraint" => Self::NamedConstraint,
            "null_constraint" => Self::NullConstraint,
            "null_hint" => Self::NullHint,
            "number" => Self::Number,
            "on_delete_action" => Self::OnDeleteAction,
            "on_update_action" => Self::OnUpdateAction,
            "op_class" => Self::OpClass,
            "optimizer_hint" => Self::OptimizerHint,
            "order_by_clause" => Self::OrderByClause,
            "order_by_clause_body" => Self::OrderByClauseBody,
            "ordered_expression" => Self::OrderedExpression,
            "parallel_hint" => Self::ParallelHint,
            "parameter" => Self::Parameter,
            "parameters" => Self::Parameters,
            "pg_command" => Self::PgCommand,
            "primary_key_constraint" => Self::PrimaryKeyConstraint,
            "references_constraint" => Self::ReferencesConstraint,
            "select_clause" => Self::SelectClause,
            "select_clause_body" => Self::SelectClauseBody,
            "select_statement" => Self::SelectStatement,
            "select_subexpression" => Self::SelectSubexpression,
            "sequence" => Self::Sequence,
            "set_clause" => Self::SetClause,
            "set_clause_body" => Self::SetClauseBody,
            "set_statement" => Self::SetStatement,
            "setof" => Self::Setof,
            "source_file" => Self::SourceFile,
            "string" => Self::String,
            "table_column" => Self::TableColumn,
            "table_constraint_check" => Self::TableConstraintCheck,
            "table_constraint_exclude" => Self::TableConstraintExclude,
            "table_constraint_foreign_key" => Self::TableConstraintForeignKey,
            "table_constraint_primary_key" => Self::TableConstraintPrimaryKey,
            "table_constraint_unique" => Self::TableConstraintUnique,
            "table_parameters" => Self::TableParameters,
            "time_zone_constraint" => Self::TimeZoneConstraint,
            "tuple" => Self::Tuple,
            "type" => Self::Type,
            "type_cast" => Self::TypeCast,
            "unique_constraint" => Self::UniqueConstraint,
            "update_statement" => Self::UpdateStatement,
            "using_clause" => Self::UsingClause,
            "values_clause" => Self::ValuesClause,
            "values_clause_body" => Self::ValuesClauseBody,
            "where_clause" => Self::WhereClause,
            _ => unreachable!(),
        }
    }
}
