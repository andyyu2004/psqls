//! generated, do not edit
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
    pub fn r#expression(&self) -> Option<Expression> {
        self.child()
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
    pub fn r#identifier(&self) -> Option<Identifier> {
        self.child()
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
    pub fn r#unique_constraint(&self) -> Option<UniqueConstraint> {
        self.child()
    }
}
impl CreateIndexStatement {
    pub fn r#identifier(&self) -> Option<Identifier> {
        self.child()
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
    pub fn r#identifier(&self) -> Option<Identifier> {
        self.child()
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
    pub fn r#identifier(&self) -> Option<Identifier> {
        self.child()
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
    pub fn r#identifier(&self) -> Option<Identifier> {
        self.child()
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
    pub fn r#type(&self) -> Option<Type> {
        self.child()
    }
}
impl Sequence {
    pub fn r#number(&self) -> Option<Number> {
        self.child()
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
    pub fn r#identifier(&self) -> Option<Identifier> {
        self.child()
    }
}
impl SetStatement {
    pub fn r#expression(&self) -> Option<Expression> {
        self.child()
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
    pub fn r#expression(&self) -> Option<Expression> {
        self.child()
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
    Expression,
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
    ParenthesizedExpression,
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
    Statement,
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
    Token,
    Err,
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
            "expression" => Self::Expression,
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
            "parenthesized_expression" => Self::ParenthesizedExpression,
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
            "statement" => Self::Statement,
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
            "ERROR" => Self::Err,
            s => unreachable!("unexpected SyntaxKind `{}`", s),
        }
    }
}
