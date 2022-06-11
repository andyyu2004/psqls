// this file is just here to ensure the visitor trait doesn't unexpectedly change
use crate::generated::*;

struct TestVisitor {}

impl Visitor for TestVisitor {
    fn visit_alter_statement(&mut self, r#alter_statement: AlterStatement) {
        for r#alter_table in r#alter_statement.r#alter_table() {
            self.visit_alter_table(r#alter_table);
        }
        for r#sequence in r#alter_statement.r#sequence() {
            self.visit_sequence(r#sequence);
        }
    }

    fn visit_alter_table(&mut self, r#alter_table: AlterTable) {
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
        for r#table_column in r#alter_table_action_add.r#table_column() {
            self.visit_table_column(r#table_column);
        }
    }

    fn visit_alter_table_action_alter_column(
        &mut self,
        r#alter_table_action_alter_column: AlterTableActionAlterColumn,
    ) {
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
    }

    fn visit_binary_expression(&mut self, r#binary_expression: BinaryExpression) {
        for r#expression in r#binary_expression.r#expression() {
            self.visit_expression(r#expression);
        }
    }

    fn visit_binary_operator(&mut self, r#binary_operator: BinaryOperator) {}

    fn visit_boolean_expression(&mut self, r#boolean_expression: BooleanExpression) {
        for r#expression in r#boolean_expression.r#expression() {
            self.visit_expression(r#expression);
        }
    }

    fn visit_check_constraint(&mut self, r#check_constraint: CheckConstraint) {
        for r#expression in r#check_constraint.r#expression() {
            self.visit_expression(r#expression);
        }
    }

    fn visit_column_default(&mut self, r#column_default: ColumnDefault) {
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
        for r#identifier in r#create_extension_statement.r#identifier() {
            self.visit_identifier(r#identifier);
        }
    }

    fn visit_create_function_parameter(
        &mut self,
        r#create_function_parameter: CreateFunctionParameter,
    ) {
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
        for r#identifier in r#create_role_statement.r#identifier() {
            self.visit_identifier(r#identifier);
        }
    }

    fn visit_create_schema_statement(&mut self, r#create_schema_statement: CreateSchemaStatement) {
        for r#identifier in r#create_schema_statement.r#identifier() {
            self.visit_identifier(r#identifier);
        }
    }

    fn visit_create_statement(&mut self, r#create_statement: CreateStatement) {
        for r#sequence in r#create_statement.r#sequence() {
            self.visit_sequence(r#sequence);
        }
    }

    fn visit_create_table_statement(&mut self, r#create_table_statement: CreateTableStatement) {
        for r#table_parameters in r#create_table_statement.r#table_parameters() {
            self.visit_table_parameters(r#table_parameters);
        }
    }

    fn visit_create_type_statement(&mut self, r#create_type_statement: CreateTypeStatement) {
        for r#identifier in r#create_type_statement.r#identifier() {
            self.visit_identifier(r#identifier);
        }
        for r#parameters in r#create_type_statement.r#parameters() {
            self.visit_parameters(r#parameters);
        }
    }

    fn visit_direction_constraint(&mut self, r#direction_constraint: DirectionConstraint) {}

    fn visit_distinct_from(&mut self, r#distinct_from: DistinctFrom) {
        for r#expression in r#distinct_from.r#expression() {
            self.visit_expression(r#expression);
        }
    }

    fn visit_dotted_name(&mut self, r#dotted_name: DottedName) {
        for r#identifier in r#dotted_name.r#identifier() {
            self.visit_identifier(r#identifier);
        }
    }

    fn visit_drop_statement(&mut self, r#drop_statement: DropStatement) {}

    fn visit_exclude_entry(&mut self, r#exclude_entry: ExcludeEntry) {
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

    fn visit_false(&mut self, r#false: False) {}

    fn visit_field_access(&mut self, r#field_access: FieldAccess) {
        for r#identifier in r#field_access.r#identifier() {
            self.visit_identifier(r#identifier);
        }
        for r#string in r#field_access.r#string() {
            self.visit_string(r#string);
        }
    }

    fn visit_from_clause(&mut self, r#from_clause: FromClause) {}

    fn visit_function_body(&mut self, r#function_body: FunctionBody) {
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
        for r#identifier in r#grant_statement.r#identifier() {
            self.visit_identifier(r#identifier);
        }
    }

    fn visit_group_by_clause(&mut self, r#group_by_clause: GroupByClause) {
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

    fn visit_initial_mode(&mut self, r#initial_mode: InitialMode) {}

    fn visit_insert_statement(&mut self, r#insert_statement: InsertStatement) {
        for r#values_clause in r#insert_statement.r#values_clause() {
            self.visit_values_clause(r#values_clause);
        }
    }

    fn visit_interval_expression(&mut self, r#interval_expression: IntervalExpression) {
        for r#string in r#interval_expression.r#string() {
            self.visit_string(r#string);
        }
    }

    fn visit_is_expression(&mut self, r#is_expression: IsExpression) {
        for r#false in r#is_expression.r#false() {
            self.visit_false(r#false);
        }
        for r#null in r#is_expression.r#null() {
            self.visit_null(r#null);
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
        for r#expression in r#join_clause.r#expression() {
            self.visit_expression(r#expression);
        }
        for r#join_type in r#join_clause.r#join_type() {
            self.visit_join_type(r#join_type);
        }
    }

    fn visit_join_type(&mut self, r#join_type: JoinType) {}

    fn visit_mode(&mut self, r#mode: Mode) {}

    fn visit_named_constraint(&mut self, r#named_constraint: NamedConstraint) {
        for r#identifier in r#named_constraint.r#identifier() {
            self.visit_identifier(r#identifier);
        }
    }

    fn visit_null(&mut self, r#null: Null) {}

    fn visit_null_constraint(&mut self, r#null_constraint: NullConstraint) {
        for r#null in r#null_constraint.r#null() {
            self.visit_null(r#null);
        }
    }

    fn visit_null_hint(&mut self, r#null_hint: NullHint) {}

    fn visit_number(&mut self, r#number: Number) {}

    fn visit_on_delete_action(&mut self, r#on_delete_action: OnDeleteAction) {}

    fn visit_on_update_action(&mut self, r#on_update_action: OnUpdateAction) {}

    fn visit_op_class(&mut self, r#op_class: OpClass) {}

    fn visit_optimizer_hint(&mut self, r#optimizer_hint: OptimizerHint) {}

    fn visit_order_by_clause(&mut self, r#order_by_clause: OrderByClause) {
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
        for r#expression in r#ordered_expression.r#expression() {
            self.visit_expression(r#expression);
        }
    }

    fn visit_parallel_hint(&mut self, r#parallel_hint: ParallelHint) {}

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

    fn visit_primary_key_constraint(&mut self, r#primary_key_constraint: PrimaryKeyConstraint) {}

    fn visit_references_constraint(&mut self, r#references_constraint: ReferencesConstraint) {
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
        for r#expression in r#set_statement.r#expression() {
            self.visit_expression(r#expression);
        }
        for r#identifier in r#set_statement.r#identifier() {
            self.visit_identifier(r#identifier);
        }
    }

    fn visit_setof(&mut self, r#setof: Setof) {
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
        for r#expression in r#table_constraint_check.r#expression() {
            self.visit_expression(r#expression);
        }
    }

    fn visit_table_constraint_exclude(
        &mut self,
        r#table_constraint_exclude: TableConstraintExclude,
    ) {
        for r#exclude_entry in r#table_constraint_exclude.exclude_entrys() {
            self.visit_exclude_entry(r#exclude_entry);
        }
    }

    fn visit_table_constraint_foreign_key(
        &mut self,
        r#table_constraint_foreign_key: TableConstraintForeignKey,
    ) {
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
    }

    fn visit_table_constraint_unique(&mut self, r#table_constraint_unique: TableConstraintUnique) {}

    fn visit_table_parameters(&mut self, r#table_parameters: TableParameters) {
        for r#table_column in r#table_parameters.r#table_column() {
            self.visit_table_column(r#table_column);
        }
    }

    fn visit_time_zone_constraint(&mut self, r#time_zone_constraint: TimeZoneConstraint) {}

    fn visit_true(&mut self, r#true: True) {}

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

    fn visit_unique_constraint(&mut self, r#unique_constraint: UniqueConstraint) {}

    fn visit_update_statement(&mut self, r#update_statement: UpdateStatement) {
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
        for r#identifier in r#using_clause.r#identifier() {
            self.visit_identifier(r#identifier);
        }
    }

    fn visit_values_clause(&mut self, r#values_clause: ValuesClause) {
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
        for r#expression in r#where_clause.r#expression() {
            self.visit_expression(r#expression);
        }
    }
}
