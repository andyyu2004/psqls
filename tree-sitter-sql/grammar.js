const PREC = {
  primary: 8,
  unary: 7,
  exp: 6,
  multiplicative: 5,
  additive: 4,
  comparative: 3,
  and: 2,
  or: 1,
};
const multiplicative_operators = ["*", "/", "%", "<<", ">>", "&"];
const additive_operators = ["+", "-", "|", "#"];
const comparative_operators = [
  "<",
  "<=",
  "<>",
  "=",
  ">",
  ">=",
  "~",
  "!~",
  "~*",
  "!~*",
];

// Generate case insentitive match for SQL keyword
// In case of multiple word keyword provide a seq matcher
function kw(keyword) {
  if (keyword.toUpperCase() != keyword) {
    throw new Error(`Expected upper case keyword got ${keyword}`);
  }
  const words = keyword.split(" ");
  const regExps = words.map(createCaseInsensitiveRegex);

  if (regExps.length == 1) {
    return alias(regExps[0], keyword)
  } else {
    return seq(...words.map(kw))
  }
}

function createOrReplace(item) {
  if (item.toUpperCase() != item) {
    throw new Error(`Expected upper case item got ${item}`);
  }
  return alias(
    seq(
      createCaseInsensitiveRegex("CREATE"),
      field("replace", optional(createCaseInsensitiveRegex("OR REPLACE"))),
      createCaseInsensitiveRegex(item),
    ),
    `CREATE_OR_REPLACE_${item}`,
  );
}

function createCaseInsensitiveRegex(word) {
  return new RegExp(
    word
      .split("")
      .map(letter => `[${letter.toLowerCase()}${letter.toUpperCase()}]`)
      .join(""),
  );
}

module.exports = grammar({
  name: "sql",
  extras: $ => [$.comment, /[\s\f\uFEFF\u2060\u200B]|\\\r?\n/],
  rules: {
    source_file: $ => seq(sep1($._statement, ";"), optional(";")),

    _statement: $ => choice(
      $.pg_command,
      $.begin_statement,
      $.commit_statement,
      $.rollback_statement,
      $.select_statement,
      $.update_statement,
      $.set_statement,
      $.insert_statement,
      $.grant_statement,
      $.drop_statement,
      $.create_statement,
      $.alter_statement,
      $.create_type_statement,
      $.create_domain_statement,
      $.create_index_statement,
      $.create_table_statement,
      $.create_function_statement,
      $.create_schema_statement,
      $.create_role_statement,
      $.create_extension_statement,
    ),

    begin_statement: $ =>
      seq(kw("BEGIN"), optional(choice(kw("WORK"), kw("TRANSACTION")))),
    commit_statement: $ =>
      seq(kw("COMMIT"), optional(choice(kw("WORK"), kw("TRANSACTION")))),
    rollback_statement: $ =>
      seq(kw("ROLLBACK"), optional(choice(kw("WORK"), kw("TRANSACTION")))),

    create_statement: $ =>
      seq(
        kw("CREATE"),
        optional(choice(kw("TEMP"), kw("TEMPORARY"))),
        choice(alias($.sequence, $.create_sequence)),
      ),
    alter_statement: $ =>
      seq(
        kw("ALTER"),
        choice(alias($.sequence, $.alter_sequence), $.alter_table),
      ),
    alter_table: $ =>
      seq(
        kw("TABLE"),
        optional(kw("IF EXISTS")),
        optional(kw("ONLY")),
        $._name,
        $.alter_table_action,
      ),
    alter_table_action_alter_column: $ =>
      seq(
        kw("ALTER COLUMN"),
        $._name,
        kw("SET DEFAULT"),
        $._column_default_expression,
      ),
    alter_table_action_add: $ =>
      seq(
        kw("ADD"),
        choice(seq(kw("COLUMN"), $.table_column), $._table_constraint),
      ),
    alter_table_action_set: $ => seq(kw("SET"), $._expression),
    alter_table_action: $ =>
      choice(
        $.alter_table_action_add,
        $.alter_table_action_alter_column,
        $.alter_table_action_set,
      ),
    sequence: $ =>
      seq(
        kw("SEQUENCE"),
        optional(seq(kw("IF"), optional(kw("NOT")), kw("EXISTS"))),
        $._name,
        optional(seq(kw("AS"), $.type)),
        repeat(
          choice(
            seq(kw("START"), kw("WITH"), $.number),
            seq(kw("INCREMENT"), optional(kw("BY")), $.number),
            seq(kw("NO"), choice(kw("MINVALUE"), kw("MAXVALUE"))),
            seq(kw("CACHE"), $.number),
            seq(kw("OWNED BY"), choice($._name)),
          ),
        ),
      ),
    pg_command: $ => seq(/\\[a-zA-Z]+/, /.*/),
    create_function_statement: $ =>
      seq(
        createOrReplace("FUNCTION"),
        $._name,
        $.create_function_parameters,
        kw("RETURNS"),
        $._create_function_return_type,
        repeat(
          choice(
            $._function_language,
            $.function_body,
            $.optimizer_hint,
            $.parallel_hint,
            $.null_hint,
          ),
        ),
      ),
    optimizer_hint: $ => choice(kw("VOLATILE"), kw("IMMUTABLE"), kw("STABLE")),
    parallel_hint: $ =>
      choice(
        kw("PARALLEL"),
        choice(kw("SAFE"), kw("UNSAFE"), kw("RESTRICTED")),
      ),
    null_hint: $ =>
      choice(
        kw("CALLED ON NULL INPUT"),
        kw("RETURNS NULL ON NULL INPUT"),
        kw("STRICT"),
      ),
    _function_language: $ =>
      seq(kw("LANGUAGE"), alias($.identifier, $.language)),
    _create_function_return_type: $ =>
      choice($._anytype, $.setof, $.constrained_type),
    setof: $ => seq(kw("SETOF"), choice($._anytype, $.constrained_type)),
    constrained_type: $ => seq(seq($._anytype, $.null_constraint)),
    create_function_parameter: $ =>
      seq(
        field(
          "argmode",
          optional(choice(kw("IN"), kw("OUT"), kw("INOUT"), kw("VARIADIC"))),
        ),
        optional($.identifier),
        choice($._anytype, $.constrained_type),
        optional(seq("=", alias($._expression, $.default))),
      ),
    create_function_parameters: $ =>
      seq("(", commaSep1($.create_function_parameter), ")"),
    function_body: $ => seq(kw("AS"), $.string),
    create_extension_statement: $ =>
      seq(kw("CREATE EXTENSION"), optional(kw("IF NOT EXISTS")), $._name),
    create_role_statement: $ =>
      seq(
        kw("CREATE ROLE"),
        $._name,
        optional(kw("WITH")),
        optional($._name),
      ),
    create_schema_statement: $ =>
      seq(kw("CREATE SCHEMA"), optional(kw("IF NOT EXISTS")), $._name),
    drop_statement: $ =>
      seq(
        kw("DROP"),
        choice("TABLE", "VIEW", "TABLESPACE", "EXTENSION", "INDEX"),
        optional(kw("IF EXISTS")),
        $._name,
      ),
    set_statement: $ =>
      seq(
        kw("SET"),
        field("scope", optional(choice(kw("SESSION"), kw("LOCAL")))),
        $.identifier,
        choice("=", kw("TO")),
        choice($._expression, kw("DEFAULT")),
      ),
    grant_statement: $ =>
      seq(
        kw("GRANT"),
        choice(
          seq(kw("ALL"), optional(kw("PRIVILEGES"))),
          repeat(
            choice(
              kw("SELECT"),
              kw("INSERT"),
              kw("UPDATE"),
              kw("DELETE"),
              kw("TRUNCATE"),
              kw("REFERENCES"),
              kw("TRIGGER"),
              kw("USAGE"),
            ),
          ),
        ),
        kw("ON"),
        field(
          "type",
          optional(
            choice(kw("SCHEMA"), kw("DATABASE"), kw("SEQUENCE"), kw("TABLE")),
          ),
        ),
        $._name,
        kw("TO"),
        choice(seq(optional(kw("GROUP")), $.identifier), kw("PUBLIC")),
        optional(kw("WITH GRANT OPTION")),
      ),
    create_domain_statement: $ =>
      seq(
        kw("CREATE DOMAIN"),
        $._name,
        optional(
          seq(
            kw("AS"),
            $._anytype,
            repeat(choice($.null_constraint, $.check_constraint)),
          ),
        ),
      ),
    create_type_statement: $ =>
      seq(kw("CREATE TYPE"), $._name, kw("AS"), $.parameters),
    create_index_with_clause: $ =>
      seq(
        kw("WITH"),
        "(",
        field("storage_parameter", $.identifier),
        "=",
        // TODO: Option value be special set e.g. accepting enum with ON, OFF, VALUE
        $._expression,
        ")",
      ),
    create_index_include_clause: $ =>
      seq(kw("INCLUDE"), "(", commaSep1($.identifier), ")"),
    create_index_statement: $ =>
      seq(
        kw("CREATE"),
        optional($.unique_constraint),
        kw("INDEX"),
        field("name", $._name),
        kw("ON"),
        field("table_name", $._name),
        optional($.using_clause),
        $.index_table_parameters,
        optional($.create_index_include_clause),
        optional($.create_index_with_clause),
        optional($.where_clause),
      ),
    table_column: $ =>
      seq(
        field("name", $._name),
        field("type", $._anytype),
        repeat(
          choice(
            $.column_default,
            $.primary_key_constraint,
            $.check_constraint,
            $.references_constraint,
            $.unique_constraint,
            $.null_constraint,
            $.named_constraint,
            $.direction_constraint,
            $.auto_increment_constraint,
            $.time_zone_constraint,
          ),
        ),
      ),
    auto_increment_constraint: $ => alias(kw("AUTO_INCREMENT"), $.auto_increment_constraint),
    direction_constraint: _ => choice(kw("ASC"), kw("DESC")),
    time_zone_constraint: _ =>
      seq(choice(kw("WITH"), kw("WITHOUT")), kw("TIME ZONE")),
    named_constraint: $ => seq("CONSTRAINT", $.identifier),
    _column_default_expression: $ =>
      choice(
        $._parenthesized_expression,
        $.string,
        $.identifier,
        $.function_call,
      ),
    column_default: $ =>
      seq(
        kw("DEFAULT"),
        // TODO: this should be specific variable-free expression https://www.postgresql.org/docs/9.1/sql-createtable.html
        // TODO: simple expression to use for check and default
        choice($._column_default_expression, $.type_cast),
      ),
    table_parameters: $ =>
      seq("(", commaSep1(choice($.table_column, $._table_constraint)), ")"),
    mode: $ => choice(kw("NOT DEFERRABLE"), kw("DEFERRABLE")),
    initial_mode: $ =>
      seq(kw("INITIALLY"), choice(kw("DEFERRED"), kw("IMMEDIATE"))),
    _table_constraint: $ =>
      seq(
        optional(seq(kw("CONSTRAINT"), field("name", $._name))),
        choice(
          $.table_constraint_foreign_key,
          $.table_constraint_unique,
          $.table_constraint_primary_key,
          $.table_constraint_check,
          $.table_constraint_exclude,
        ),
        optional($.mode),
        optional($.initial_mode),
      ),
    table_constraint_check: $ => seq(kw("CHECK"), $._expression),
    op_class: $ => $._name,
    exclude_entry: $ =>
      seq(
        $._name,
        optional($.op_class),
        optional(seq(kw("WITH"), $.binary_operator)),
      ),
    table_constraint_exclude: $ =>
      seq(
        kw("EXCLUDE"),
        optional(seq(kw("USING"), $._name)),
        "(",
        commaSep1($.exclude_entry),
        ")",
      ),
    table_constraint_foreign_key: $ =>
      seq(
        kw("FOREIGN KEY"),
        "(",
        commaSep1($.identifier),
        ")",
        $.references_constraint,
      ),
    table_constraint_unique: $ =>
      seq(kw("UNIQUE"), "(", commaSep1($._name), ")"),
    table_constraint_primary_key: $ =>
      seq(kw("PRIMARY KEY"), "(", commaSep1($._name), ")"),
    primary_key_constraint: $ => kw("PRIMARY KEY"),
    create_table_statement: $ =>
      seq(
        kw("CREATE"),
        optional(kw("TEMPORARY")),
        kw("TABLE"),
        optional(kw("IF NOT EXISTS")),
        $._name,
        $.table_parameters,
      ),
    using_clause: $ => seq(kw("USING"), field("method", $.identifier)),
    index_table_parameters: $ =>
      seq(
        "(",
        commaSep1(
          seq(
            choice($._expression, $.ordered_expression),
            optional($.op_class),
          ),
        ),
        ")",
      ),

    // SELECT
    select_statement: $ =>
      seq(
        $.select_clause,
        optional($.from_clause),
        optional(repeat($.join_clause)),
        optional($.where_clause),
        optional($.group_by_clause),
        optional($.order_by_clause),
      ),
    group_by_clause_body: $ => commaSep1($._expression),
    group_by_clause: $ => seq(kw("GROUP BY"), $.group_by_clause_body),
    order_by_clause_body: $ => commaSep1($._expression),
    order_by_clause: $ => seq(kw("ORDER BY"), $.order_by_clause_body),
    where_clause: $ => seq(kw("WHERE"), $._expression),
    _aliased_expression: $ =>
      seq($._expression, optional(kw("AS")), $.identifier),
    _aliasable_expression: $ =>
      choice($._expression, alias($._aliased_expression, $.alias)),
    select_clause_body: $ => commaSep1($._aliasable_expression),
    select_clause: $ =>
      prec.left(seq(kw("SELECT"), optional($.select_clause_body))),
    from_clause: $ => seq(kw("FROM"), commaSep1($._aliasable_expression)),
    join_type: $ =>
      seq(
        choice(
          kw("INNER"),
          seq(
            choice(kw("LEFT"), kw("RIGHT"), kw("FULL")),
            optional(kw("OUTER")),
          ),
        ),
      ),
    join_clause: $ =>
      seq(
        optional($.join_type),
        kw("JOIN"),
        $._name,
        kw("ON"),
        $._expression,
      ),
    select_subexpression: $ => seq("(", $.select_statement, ")"),

    // UPDATE
    update_statement: $ =>
      seq(kw("UPDATE"), $.identifier, $.set_clause, optional($.where_clause)),

    set_clause: $ => seq(kw("SET"), $.set_clause_body),
    set_clause_body: $ => seq(commaSep1($.assigment_expression)),
    assigment_expression: $ => seq($.identifier, "=", $._expression),

    // INSERT
    insert_statement: $ =>
      seq(
        kw("INSERT"),
        kw("INTO"),
        $._name,
        choice($.values_clause, $.select_statement),
      ),
    values_clause: $ => seq(kw("VALUES"), "(", $.values_clause_body, ")"),
    values_clause_body: $ => commaSep1($._expression),
    in_expression: $ =>
      prec.left(1, seq($._expression, optional(kw("NOT")), kw("IN"), $.tuple)),
    tuple: $ =>
      seq(
        // TODO: maybe collapse with function arguments, but make sure to preserve clarity
        "(",
        field("elements", commaSep1($._expression)),
        ")",
      ),
    // TODO: named constraints
    references_constraint: $ =>
      seq(
        kw("REFERENCES"),
        $._name,
        optional(seq("(", commaSep1($.identifier), ")")),
        // seems like a case for https://github.com/tree-sitter/tree-sitter/issues/130
        repeat(choice($.on_update_action, $.on_delete_action)),
      ),
    on_update_action: $ =>
      seq(kw("ON UPDATE"), field("action", $._constraint_action)),
    on_delete_action: $ =>
      seq(kw("ON DELETE"), field("action", $._constraint_action)),
    _constraint_action: $ =>
      choice(kw("RESTRICT"), kw("CASCADE"), kw("SET NULL")),
    unique_constraint: $ => alias(kw("UNIQUE"), $.unique_constraint),
    null_constraint: $ => seq(optional(kw("NOT")), $.NULL),
    check_constraint: $ => seq(kw("CHECK"), $._expression),
    _constraint: $ =>
      seq(
        choice($.null_constraint, $.check_constraint),
        optional($.check_constraint),
      ),
    parameter: $ => seq($.identifier, choice($._anytype, $.constrained_type)),
    parameters: $ => seq("(", commaSep1($.parameter), ")"),
    function_call: $ =>
      seq(
        field("function", $.identifier),
        "(",
        optional(field("arguments", commaSep1($._expression))),
        ")",
      ),
    _parenthesized_expression: $ => seq("(", $._expression, ")"),
    is_expression: $ =>
      prec.left(
        1,
        seq(
          $._expression,
          kw("IS"),
          optional(kw("NOT")),
          choice($.NULL, $.TRUE, $.FALSE, $.distinct_from),
        ),
      ),
    distinct_from: $ => prec.left(seq(kw("DISTINCT FROM"), $._expression)),
    boolean_expression: $ =>
      choice(
        prec.left(PREC.unary, seq(kw("NOT"), $._expression)),
        prec.left(PREC.and, seq($._expression, kw("AND"), $._expression)),
        prec.left(PREC.or, seq($._expression, kw("OR"), $._expression)),
      ),
    NULL: $ => kw("NULL"),
    TRUE: $ => kw("TRUE"),
    FALSE: $ => kw("FALSE"),
    number: $ => /\d+/,
    _quoted_identifier: $ =>
      choice(
        seq("`", field("name", /[^`]*/), "`"), // MySQL style quoting
        seq('"', field("name", /[^"]*/), '"'), // ANSI QUOTES
      ),
    identifier: $ => choice($._quoted_identifier, /[a-zA-Z0-9_]+/),
    dotted_name: $ => prec.left(PREC.primary, sep2($.identifier, ".")),
    _name: $ => seq(choice($.identifier, $.dotted_name)),
    type: $ => seq($._name, optional(seq("(", $.number, ")"))),
    string_content: _ => /[^']*/,
    raw_string_content: _ => /(\$?[^$]+)+/,
    string: $ =>
      choice(
        seq("'", field("content", $.string_content), "'"),
        seq("$$", field("content", $.raw_string_content), "$$"), // FIXME empty string test, maybe read a bit more into c comments answer
      ),
    field_access: $ => seq($.identifier, "->>", $.string),
    ordered_expression: $ =>
      seq($._expression, field("order", choice(kw("ASC"), kw("DESC")))),
    _anytype: $ => choice($.type, alias(seq($._anytype, "[", "]"), "array_type")),
    type_cast: $ =>
      seq(
        // TODO: should be moved to basic expression or something
        choice(
          $._parenthesized_expression,
          $.string,
          $.identifier,
          $.function_call,
        ),
        "::",
        field("type", $._anytype),
      ),
    // http://stackoverflow.com/questions/13014947/regex-to-match-a-c-style-multiline-comment/36328890#36328890
    comment: $ =>
      token(
        choice(seq("--", /.*/), seq("/*", /[^*]*\*+([^/*][^*]*\*+)*/, "/")),
      ),
    array_element_access: $ =>
      seq(choice($.identifier, $.argument_reference), "[", $._expression, "]"),

    unary_expression: $ =>
      prec(
        PREC.unary,
        seq(
          field(
            "operator",
            choice(
              "+",
              "-",
              "!!", // Factorial op (Removed in Postgres >= 14)
              "~", // Bitwise not
              "@", // Absolute value
              "|/", // square root
              "||/", // cube root
            ),
          ),
          field("operand", $._expression),
        ),
      ),

    binary_expression: $ => {
      const table = [
        [PREC.exp, "^"],
        [PREC.multiplicative, choice(...multiplicative_operators)],
        [PREC.additive, choice(...additive_operators)],
        [PREC.comparative, choice(...comparative_operators)],
      ];

      return choice(
        ...table.map(([precedence, operator]) =>
          prec.left(
            precedence,
            seq(
              field("left", $._expression),
              field("operator", operator),
              field("right", $._expression),
            ),
          ),
        ),
      );
    },

    binary_operator: $ => choice("=", "&&", "||"),
    asterisk_expression: $ => choice("*", seq($._name, ".*")),
    interval_expression: $ => seq(token(prec(1, kw("INTERVAL"))), $.string),
    argument_reference: $ => seq("$", /\d+/),
    _expression: $ =>
      choice(
        $.interval_expression,
        $.function_call,
        $.string,
        $.field_access,
        $.TRUE,
        $.FALSE,
        $.NULL,
        $.asterisk_expression,
        $._name,
        $.number,
        $.in_expression,
        $.is_expression,
        $.boolean_expression,
        $._parenthesized_expression,
        $.type_cast,
        $.unary_expression,
        $.binary_expression,
        $.array_element_access,
        $.argument_reference,
        $.select_subexpression,
      ),
  },
});

function commaSep1(rule) {
  return sep1(rule, ",");
}

function sep1(rule, separator) {
  return seq(rule, repeat(seq(separator, rule)));
}

function sep2(rule, separator) {
  return seq(rule, repeat1(seq(separator, rule)));
}
