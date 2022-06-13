#![allow(unused)]

// this file is just here to ensure the visitor trait doesn't unexpectedly change
use crate::generated::*;

struct TestVisitor {}

impl Visitor for TestVisitor {
    fn visit_kw(&mut self, kw: crate::SyntaxToken) {}

    fn visit_aliasable_expression(&mut self, r#aliasable_expression: AliasableExpression) {
        for r#aliased_expression in r#aliasable_expression.r#aliased_expression() {
            self.visit_aliased_expression(r#aliased_expression);
        }
        for r#expression in r#aliasable_expression.r#expression() {
            self.visit_expression(r#expression);
        }
    }

    fn visit_aliased_expression(&mut self, r#aliased_expression: AliasedExpression) {
        if let Some(kw) = r#aliased_expression.as_kw() {
            self.visit_kw(kw);
        }
        for r#expression in r#aliased_expression.r#expression() {
            self.visit_expression(r#expression);
        }
        for r#identifier in r#aliased_expression.r#identifier() {
            self.visit_identifier(r#identifier);
        }
    }

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
        for r#name in r#alter_table.r#name() {
            self.visit_name(r#name);
        }
        for r#alter_table_action in r#alter_table.r#alter_table_action() {
            self.visit_alter_table_action(r#alter_table_action);
        }
    }

    fn visit_alter_table_action(&mut self, r#alter_table_action: AlterTableAction) {
        match r#alter_table_action {
            AlterTableAction::AlterTableActionAdd(node) => self.visit_alter_table_action_add(node),
            AlterTableAction::AlterTableActionAlterColumn(node) => {
                self.visit_alter_table_action_alter_column(node)
            }
            AlterTableAction::AlterTableActionSet(node) => self.visit_alter_table_action_set(node),
        }
    }

    fn visit_alter_table_action_add(&mut self, r#alter_table_action_add: AlterTableActionAdd) {
        if let Some(kw) = r#alter_table_action_add.add_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#alter_table_action_add.column_kw() {
            self.visit_kw(kw);
        }
        for r#table_constraint in r#alter_table_action_add.r#table_constraint() {
            self.visit_table_constraint(r#table_constraint);
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
        for r#column_default_expression in
            r#alter_table_action_alter_column.r#column_default_expression()
        {
            self.visit_column_default_expression(r#column_default_expression);
        }
        for r#name in r#alter_table_action_alter_column.r#name() {
            self.visit_name(r#name);
        }
    }

    fn visit_alter_table_action_set(&mut self, r#alter_table_action_set: AlterTableActionSet) {
        if let Some(kw) = r#alter_table_action_set.set_kw() {
            self.visit_kw(kw);
        }
        for r#expression in r#alter_table_action_set.r#expression() {
            self.visit_expression(r#expression);
        }
    }

    fn visit_anytype(&mut self, r#anytype: Anytype) {
        for r#anytype in r#anytype.r#anytype() {
            self.visit_anytype(r#anytype);
        }
        for r#type in r#anytype.r#type() {
            self.visit_type(r#type);
        }
    }

    fn visit_argument_reference(&mut self, r#argument_reference: ArgumentReference) {}

    fn visit_array_element_access(&mut self, r#array_element_access: ArrayElementAccess) {
        for r#expression in r#array_element_access.r#expression() {
            self.visit_expression(r#expression);
        }
        for r#argument_reference in r#array_element_access.r#argument_reference() {
            self.visit_argument_reference(r#argument_reference);
        }
        for r#identifier in r#array_element_access.r#identifier() {
            self.visit_identifier(r#identifier);
        }
    }

    fn visit_assigment_expression(&mut self, r#assigment_expression: AssigmentExpression) {
        for r#expression in r#assigment_expression.r#expression() {
            self.visit_expression(r#expression);
        }
        for r#identifier in r#assigment_expression.r#identifier() {
            self.visit_identifier(r#identifier);
        }
    }

    fn visit_asterisk_expression(&mut self, r#asterisk_expression: AsteriskExpression) {
        for r#name in r#asterisk_expression.r#name() {
            self.visit_name(r#name);
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

    fn visit_begin_statement(&mut self, r#begin_statement: BeginStatement) {
        if let Some(kw) = r#begin_statement.begin_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#begin_statement.transaction_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#begin_statement.work_kw() {
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
        for r#column_default_expression in r#column_default.r#column_default_expression() {
            self.visit_column_default_expression(r#column_default_expression);
        }
        for r#type_cast in r#column_default.r#type_cast() {
            self.visit_type_cast(r#type_cast);
        }
    }

    fn visit_column_default_expression(
        &mut self,
        r#column_default_expression: ColumnDefaultExpression,
    ) {
        match r#column_default_expression {
            ColumnDefaultExpression::ParenthesizedExpression(node) => {
                self.visit_parenthesized_expression(node)
            }
            ColumnDefaultExpression::String(node) => self.visit_string(node),
            ColumnDefaultExpression::Identifier(node) => self.visit_identifier(node),
            ColumnDefaultExpression::FunctionCall(node) => self.visit_function_call(node),
        }
    }

    fn visit_comment(&mut self, r#comment: Comment) {}

    fn visit_commit_statement(&mut self, r#commit_statement: CommitStatement) {
        if let Some(kw) = r#commit_statement.commit_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#commit_statement.transaction_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#commit_statement.work_kw() {
            self.visit_kw(kw);
        }
    }

    fn visit_constrained_type(&mut self, r#constrained_type: ConstrainedType) {
        for r#anytype in r#constrained_type.r#anytype() {
            self.visit_anytype(r#anytype);
        }
        for r#null_constraint in r#constrained_type.r#null_constraint() {
            self.visit_null_constraint(r#null_constraint);
        }
    }

    fn visit_constraint(&mut self, r#constraint: Constraint) {
        for r#check_constraint in r#constraint.r#check_constraint() {
            self.visit_check_constraint(r#check_constraint);
        }
        for r#null_constraint in r#constraint.r#null_constraint() {
            self.visit_null_constraint(r#null_constraint);
        }
    }

    fn visit_constraint_action(&mut self, r#constraint_action: ConstraintAction) {
        if let Some(kw) = r#constraint_action.cascade_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#constraint_action.null_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#constraint_action.restrict_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#constraint_action.set_kw() {
            self.visit_kw(kw);
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
        for r#anytype in r#create_domain_statement.r#anytype() {
            self.visit_anytype(r#anytype);
        }
        for r#name in r#create_domain_statement.r#name() {
            self.visit_name(r#name);
        }
        for r#check_constraint in r#create_domain_statement.r#check_constraint() {
            self.visit_check_constraint(r#check_constraint);
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
        for r#name in r#create_extension_statement.r#name() {
            self.visit_name(r#name);
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
        for r#anytype in r#create_function_parameter.r#anytype() {
            self.visit_anytype(r#anytype);
        }
        for r#expression in r#create_function_parameter.r#expression() {
            self.visit_expression(r#expression);
        }
        for r#constrained_type in r#create_function_parameter.r#constrained_type() {
            self.visit_constrained_type(r#constrained_type);
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

    fn visit_create_function_return_type(
        &mut self,
        r#create_function_return_type: CreateFunctionReturnType,
    ) {
        match r#create_function_return_type {
            CreateFunctionReturnType::Anytype(node) => self.visit_anytype(node),
            CreateFunctionReturnType::Setof(node) => self.visit_setof(node),
            CreateFunctionReturnType::ConstrainedType(node) => self.visit_constrained_type(node),
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
        for r#create_function_return_type in
            r#create_function_statement.r#create_function_return_type()
        {
            self.visit_create_function_return_type(r#create_function_return_type);
        }
        for r#function_language in r#create_function_statement.r#function_language() {
            self.visit_function_language(r#function_language);
        }
        for r#name in r#create_function_statement.r#name() {
            self.visit_name(r#name);
        }
        for r#create_function_parameters in
            r#create_function_statement.r#create_function_parameters()
        {
            self.visit_create_function_parameters(r#create_function_parameters);
        }
        for r#function_body in r#create_function_statement.r#function_body() {
            self.visit_function_body(r#function_body);
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

    fn visit_create_index_include_clause(
        &mut self,
        r#create_index_include_clause: CreateIndexIncludeClause,
    ) {
        if let Some(kw) = r#create_index_include_clause.include_kw() {
            self.visit_kw(kw);
        }
        for r#identifier in r#create_index_include_clause.identifiers() {
            self.visit_identifier(r#identifier);
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
        for r#name in r#create_index_statement.r#name() {
            self.visit_name(r#name);
        }
        for r#create_index_include_clause in
            r#create_index_statement.r#create_index_include_clause()
        {
            self.visit_create_index_include_clause(r#create_index_include_clause);
        }
        for r#create_index_with_clause in r#create_index_statement.r#create_index_with_clause() {
            self.visit_create_index_with_clause(r#create_index_with_clause);
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

    fn visit_create_index_with_clause(
        &mut self,
        r#create_index_with_clause: CreateIndexWithClause,
    ) {
        if let Some(kw) = r#create_index_with_clause.with_kw() {
            self.visit_kw(kw);
        }
        for r#expression in r#create_index_with_clause.r#expression() {
            self.visit_expression(r#expression);
        }
        for r#identifier in r#create_index_with_clause.r#identifier() {
            self.visit_identifier(r#identifier);
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
        for r#name in r#create_role_statement.r#name() {
            self.visit_name(r#name);
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
        for r#name in r#create_schema_statement.r#name() {
            self.visit_name(r#name);
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
        for r#name in r#create_table_statement.r#name() {
            self.visit_name(r#name);
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
        for r#name in r#create_type_statement.r#name() {
            self.visit_name(r#name);
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
        if let Some(kw) = r#drop_statement.if_kw() {
            self.visit_kw(kw);
        }
        for r#name in r#drop_statement.r#name() {
            self.visit_name(r#name);
        }
    }

    fn visit_exclude_entry(&mut self, r#exclude_entry: ExcludeEntry) {
        if let Some(kw) = r#exclude_entry.with_kw() {
            self.visit_kw(kw);
        }
        for r#name in r#exclude_entry.r#name() {
            self.visit_name(r#name);
        }
        for r#binary_operator in r#exclude_entry.r#binary_operator() {
            self.visit_binary_operator(r#binary_operator);
        }
        for r#op_class in r#exclude_entry.r#op_class() {
            self.visit_op_class(r#op_class);
        }
    }

    fn visit_expression(&mut self, r#expression: Expression) {
        match r#expression {
            Expression::IntervalExpression(node) => self.visit_interval_expression(node),
            Expression::FunctionCall(node) => self.visit_function_call(node),
            Expression::String(node) => self.visit_string(node),
            Expression::FieldAccess(node) => self.visit_field_access(node),
            Expression::True(node) => self.visit_true(node),
            Expression::False(node) => self.visit_false(node),
            Expression::Null(node) => self.visit_null(node),
            Expression::AsteriskExpression(node) => self.visit_asterisk_expression(node),
            Expression::Name(node) => self.visit_name(node),
            Expression::Number(node) => self.visit_number(node),
            Expression::InExpression(node) => self.visit_in_expression(node),
            Expression::IsExpression(node) => self.visit_is_expression(node),
            Expression::BooleanExpression(node) => self.visit_boolean_expression(node),
            Expression::ParenthesizedExpression(node) => self.visit_parenthesized_expression(node),
            Expression::TypeCast(node) => self.visit_type_cast(node),
            Expression::UnaryExpression(node) => self.visit_unary_expression(node),
            Expression::BinaryExpression(node) => self.visit_binary_expression(node),
            Expression::ArrayElementAccess(node) => self.visit_array_element_access(node),
            Expression::ArgumentReference(node) => self.visit_argument_reference(node),
            Expression::SelectSubexpression(node) => self.visit_select_subexpression(node),
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
        for r#aliasable_expression in r#from_clause.r#aliasable_expression() {
            self.visit_aliasable_expression(r#aliasable_expression);
        }
    }

    fn visit_function_body(&mut self, r#function_body: FunctionBody) {
        if let Some(kw) = r#function_body.as_kw() {
            self.visit_kw(kw);
        }
        for r#string in r#function_body.r#string() {
            self.visit_string(r#string);
        }
    }

    fn visit_function_call(&mut self, r#function_call: FunctionCall) {
        for r#expression in r#function_call.r#expression() {
            self.visit_expression(r#expression);
        }
        for r#identifier in r#function_call.r#identifier() {
            self.visit_identifier(r#identifier);
        }
    }

    fn visit_function_language(&mut self, r#function_language: FunctionLanguage) {
        if let Some(kw) = r#function_language.language_kw() {
            self.visit_kw(kw);
        }
        for r#identifier in r#function_language.r#identifier() {
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
        for r#name in r#grant_statement.r#name() {
            self.visit_name(r#name);
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
        for r#expression in r#group_by_clause_body.r#expression() {
            self.visit_expression(r#expression);
        }
    }

    fn visit_identifier(&mut self, r#identifier: Identifier) {
        for r#quoted_identifier in r#identifier.r#quoted_identifier() {
            self.visit_quoted_identifier(r#quoted_identifier);
        }
    }

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
        for r#name in r#insert_statement.r#name() {
            self.visit_name(r#name);
        }
        for r#select_statement in r#insert_statement.r#select_statement() {
            self.visit_select_statement(r#select_statement);
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
        for r#expression in r#is_expression.r#expression() {
            self.visit_expression(r#expression);
        }
        for r#distinct_from in r#is_expression.r#distinct_from() {
            self.visit_distinct_from(r#distinct_from);
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
        for r#name in r#join_clause.r#name() {
            self.visit_name(r#name);
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

    fn visit_name(&mut self, r#name: Name) {
        for r#dotted_name in r#name.r#dotted_name() {
            self.visit_dotted_name(r#dotted_name);
        }
        for r#identifier in r#name.r#identifier() {
            self.visit_identifier(r#identifier);
        }
    }

    fn visit_named_constraint(&mut self, r#named_constraint: NamedConstraint) {
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
        for r#constraint_action in r#on_delete_action.r#constraint_action() {
            self.visit_constraint_action(r#constraint_action);
        }
    }

    fn visit_on_update_action(&mut self, r#on_update_action: OnUpdateAction) {
        if let Some(kw) = r#on_update_action.on_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#on_update_action.update_kw() {
            self.visit_kw(kw);
        }
        for r#constraint_action in r#on_update_action.r#constraint_action() {
            self.visit_constraint_action(r#constraint_action);
        }
    }

    fn visit_op_class(&mut self, r#op_class: OpClass) {
        for r#name in r#op_class.r#name() {
            self.visit_name(r#name);
        }
    }

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
        for r#expression in r#order_by_clause_body.r#expression() {
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
        for r#anytype in r#parameter.r#anytype() {
            self.visit_anytype(r#anytype);
        }
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

    fn visit_quoted_identifier(&mut self, r#quoted_identifier: QuotedIdentifier) {}

    fn visit_references_constraint(&mut self, r#references_constraint: ReferencesConstraint) {
        if let Some(kw) = r#references_constraint.references_kw() {
            self.visit_kw(kw);
        }
        for r#name in r#references_constraint.r#name() {
            self.visit_name(r#name);
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

    fn visit_rollback_statement(&mut self, r#rollback_statement: RollbackStatement) {
        if let Some(kw) = r#rollback_statement.rollback_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#rollback_statement.transaction_kw() {
            self.visit_kw(kw);
        }
        if let Some(kw) = r#rollback_statement.work_kw() {
            self.visit_kw(kw);
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

    fn visit_select_clause_body(&mut self, r#select_clause_body: SelectClauseBody) {
        for r#aliasable_expression in r#select_clause_body.r#aliasable_expression() {
            self.visit_aliasable_expression(r#aliasable_expression);
        }
    }

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
        for r#name in r#sequence.r#name() {
            self.visit_name(r#name);
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
        for r#anytype in r#setof.r#anytype() {
            self.visit_anytype(r#anytype);
        }
        for r#constrained_type in r#setof.r#constrained_type() {
            self.visit_constrained_type(r#constrained_type);
        }
    }

    fn visit_source_file(&mut self, r#source_file: SourceFile) {
        for r#statement in r#source_file.r#statement() {
            self.visit_statement(r#statement);
        }
    }

    fn visit_statement(&mut self, r#statement: Statement) {
        match r#statement {
            Statement::PgCommand(node) => self.visit_pg_command(node),
            Statement::BeginStatement(node) => self.visit_begin_statement(node),
            Statement::CommitStatement(node) => self.visit_commit_statement(node),
            Statement::RollbackStatement(node) => self.visit_rollback_statement(node),
            Statement::SelectStatement(node) => self.visit_select_statement(node),
            Statement::UpdateStatement(node) => self.visit_update_statement(node),
            Statement::SetStatement(node) => self.visit_set_statement(node),
            Statement::InsertStatement(node) => self.visit_insert_statement(node),
            Statement::GrantStatement(node) => self.visit_grant_statement(node),
            Statement::DropStatement(node) => self.visit_drop_statement(node),
            Statement::CreateStatement(node) => self.visit_create_statement(node),
            Statement::AlterStatement(node) => self.visit_alter_statement(node),
            Statement::CreateTypeStatement(node) => self.visit_create_type_statement(node),
            Statement::CreateDomainStatement(node) => self.visit_create_domain_statement(node),
            Statement::CreateIndexStatement(node) => self.visit_create_index_statement(node),
            Statement::CreateTableStatement(node) => self.visit_create_table_statement(node),
            Statement::CreateFunctionStatement(node) => self.visit_create_function_statement(node),
            Statement::CreateSchemaStatement(node) => self.visit_create_schema_statement(node),
            Statement::CreateRoleStatement(node) => self.visit_create_role_statement(node),
            Statement::CreateExtensionStatement(node) => {
                self.visit_create_extension_statement(node)
            }
        }
    }

    fn visit_string(&mut self, r#string: String) {}

    fn visit_table_column(&mut self, r#table_column: TableColumn) {
        for r#anytype in r#table_column.r#anytype() {
            self.visit_anytype(r#anytype);
        }
        for r#name in r#table_column.r#name() {
            self.visit_name(r#name);
        }
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

    fn visit_table_constraint(&mut self, r#table_constraint: TableConstraint) {
        if let Some(kw) = r#table_constraint.constraint_kw() {
            self.visit_kw(kw);
        }
        for r#name in r#table_constraint.r#name() {
            self.visit_name(r#name);
        }
        for r#initial_mode in r#table_constraint.r#initial_mode() {
            self.visit_initial_mode(r#initial_mode);
        }
        for r#mode in r#table_constraint.r#mode() {
            self.visit_mode(r#mode);
        }
        for r#table_constraint_check in r#table_constraint.r#table_constraint_check() {
            self.visit_table_constraint_check(r#table_constraint_check);
        }
        for r#table_constraint_exclude in r#table_constraint.r#table_constraint_exclude() {
            self.visit_table_constraint_exclude(r#table_constraint_exclude);
        }
        for r#table_constraint_foreign_key in r#table_constraint.r#table_constraint_foreign_key() {
            self.visit_table_constraint_foreign_key(r#table_constraint_foreign_key);
        }
        for r#table_constraint_primary_key in r#table_constraint.r#table_constraint_primary_key() {
            self.visit_table_constraint_primary_key(r#table_constraint_primary_key);
        }
        for r#table_constraint_unique in r#table_constraint.r#table_constraint_unique() {
            self.visit_table_constraint_unique(r#table_constraint_unique);
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
        for r#name in r#table_constraint_exclude.r#name() {
            self.visit_name(r#name);
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
        for r#name in r#table_constraint_primary_key.r#name() {
            self.visit_name(r#name);
        }
    }

    fn visit_table_constraint_unique(&mut self, r#table_constraint_unique: TableConstraintUnique) {
        if let Some(kw) = r#table_constraint_unique.unique_kw() {
            self.visit_kw(kw);
        }
        for r#name in r#table_constraint_unique.r#name() {
            self.visit_name(r#name);
        }
    }

    fn visit_table_parameters(&mut self, r#table_parameters: TableParameters) {
        for r#table_constraint in r#table_parameters.r#table_constraint() {
            self.visit_table_constraint(r#table_constraint);
        }
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
        for r#expression in r#tuple.r#expression() {
            self.visit_expression(r#expression);
        }
    }

    fn visit_type(&mut self, r#type: Type) {
        for r#name in r#type.r#name() {
            self.visit_name(r#name);
        }
        for r#number in r#type.r#number() {
            self.visit_number(r#number);
        }
    }

    fn visit_type_cast(&mut self, r#type_cast: TypeCast) {
        for r#anytype in r#type_cast.r#anytype() {
            self.visit_anytype(r#anytype);
        }
        for r#parenthesized_expression in r#type_cast.r#parenthesized_expression() {
            self.visit_parenthesized_expression(r#parenthesized_expression);
        }
        for r#function_call in r#type_cast.r#function_call() {
            self.visit_function_call(r#function_call);
        }
        for r#identifier in r#type_cast.r#identifier() {
            self.visit_identifier(r#identifier);
        }
        for r#string in r#type_cast.r#string() {
            self.visit_string(r#string);
        }
    }

    fn visit_unary_expression(&mut self, r#unary_expression: UnaryExpression) {
        for r#expression in r#unary_expression.r#expression() {
            self.visit_expression(r#expression);
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
        for r#expression in r#values_clause_body.r#expression() {
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
