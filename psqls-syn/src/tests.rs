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
fn test_parse_error() {
    let db = TestDB::from_str("foo", "select * from");
    let parsed = db.parse("foo".into());
    expect![[r#"
        SourceFile(
            SourceFile@0..11
              SelectStatement@0..7
                SelectClause@0..7
                  SelectKw@0..6 "select"
                  SelectClauseBody@6..7
                    AsteriskExpression@6..7
                      Token@6..7 "*"
              Err@7..11
                FromKw@7..11 "from"
            ,
        )
    "#]]
    .assert_debug_eq(&parsed.root());
}

#[test]
fn test_parse() {
    let db = TestDB::from_str("foo", "select * from table");
    let parsed = db.parse("foo".into());
    expect![[r#"
        SourceFile(
            SourceFile@0..11
              SelectStatement@0..11
                SelectClause@0..7
                  SelectKw@0..6 "select"
                  SelectClauseBody@6..7
                    AsteriskExpression@6..7
                      Token@6..7 "*"
                FromClause@7..11
                  FromKw@7..11 "from"
                  Identifier@11..11
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
