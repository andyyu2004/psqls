use std::sync::Arc;

use expect_test::expect;

use crate::{parse, SyntaxDatabase, SyntaxDatabaseStorage};

#[derive(Default)]
#[salsa::database(SyntaxDatabaseStorage)]
struct TestDB {
    storage: salsa::Storage<TestDB>,
}

impl TestDB {
    pub fn from_str(url: impl Into<Arc<str>>, s: impl Into<Arc<str>>) -> Self {
        let mut db = Self::default();
        db.set_text(url.into(), s.into());
        db
    }
}

impl salsa::Database for TestDB {}

#[test]
fn test_parse_preserves_whitespace() {
    let sql = " select  *   from  bar;
    ";
    let db = TestDB::from_str("foo", sql);
    let parsed = db.parse("foo".into());
    expect![[r#"
        SourceFile(
            SourceFile@0..23
              SelectStatement@0..22
                SelectClause@0..10
                  Whitespace@0..1 " "
                  SelectKw@1..7 "select"
                  SelectClauseBody@7..10
                    AsteriskExpression@7..10
                      Whitespace@7..9 "  "
                      Token@9..10 "*"
                FromClause@10..22
                  Whitespace@10..13 "   "
                  FromKw@13..17 "from"
                  Whitespace@17..19 "  "
                  Identifier@19..22 "bar"
              Token@22..23 ";"
            ,
        )
    "#]]
    .assert_debug_eq(&parsed.root())
}

#[test]
fn test_parse_create_table() {
    let db = TestDB::from_str("foo", "create table bar (id uuid PRIMARY KEY)");
    let parsed = db.parse("foo".into());
    expect![[r#"
        SourceFile(
            SourceFile@0..38
              CreateTableStatement@0..38
                CreateKw@0..6 "create"
                Whitespace@6..7 " "
                TableKw@7..12 "table"
                Whitespace@12..13 " "
                Identifier@13..16 "bar"
                TableParameters@16..38
                  Whitespace@16..17 " "
                  Token@17..18 "("
                  TableColumn@18..37
                    Identifier@18..20 "id"
                    Type@20..25
                      Whitespace@20..21 " "
                      Identifier@21..25 "uuid"
                    PrimaryKeyConstraint@25..37
                      Whitespace@25..26 " "
                      PrimaryKw@26..33 "PRIMARY"
                      Whitespace@33..34 " "
                      KeyKw@34..37 "KEY"
                  Token@37..38 ")"
            ,
        )
    "#]]
    .assert_debug_eq(&parsed.root());
}

#[test]
fn test_parse_error() {
    let db = TestDB::from_str("foo", "select * from");
    let parsed = db.parse("foo".into());
    expect![[r#"
        SourceFile(
            SourceFile@0..13
              SelectStatement@0..8
                SelectClause@0..8
                  SelectKw@0..6 "select"
                  SelectClauseBody@6..8
                    AsteriskExpression@6..8
                      Whitespace@6..7 " "
                      Token@7..8 "*"
              Err@8..13
                Whitespace@8..9 " "
                FromKw@9..13 "from"
            ,
        )
    "#]]
    .assert_debug_eq(&parsed.root());
}

#[test]
fn test_parse() {
    let db = TestDB::from_str("foo", "  select  * from table  ");
    let parsed = db.parse("foo".into());
    expect![[r#"
        SourceFile(
            SourceFile@0..22
              SelectStatement@0..22
                SelectClause@0..11
                  Whitespace@0..2 "  "
                  SelectKw@2..8 "select"
                  SelectClauseBody@8..11
                    AsteriskExpression@8..11
                      Whitespace@8..10 "  "
                      Token@10..11 "*"
                FromClause@11..22
                  Whitespace@11..12 " "
                  FromKw@12..16 "from"
                  Whitespace@16..17 " "
                  Identifier@17..22 "table"
            ,
        )
    "#]]
    .assert_debug_eq(&parsed.root());
}

#[test]
fn test_ts_parse() {
    let tree = parse("select * from table");
    expect!["(source_file (select_statement (select_clause (select_clause_body (asterisk_expression))) (from_clause (identifier))))"]
        .assert_eq(&tree.root_node().to_sexp());
}

#[test]
fn test_ts_parse_create_table() {
    let tree = parse("create table bar (id uuid PRIMARY KEY)");
    expect!["(source_file (create_table_statement (identifier) (table_parameters (table_column name: (identifier) type: (type (identifier)) (primary_key_constraint)))))"]
        .assert_eq(&tree.root_node().to_sexp());
}
