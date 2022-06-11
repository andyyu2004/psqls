use std::sync::Arc;

use expect_test::expect;

use crate::{parse, SyntaxDatabase, SyntaxDatabaseStorage};

#[derive(Default)]
#[salsa::database(SyntaxDatabaseStorage)]
struct TestDB {
    storage: salsa::Storage<TestDB>,
}

impl TestDB {
    pub fn from_str(url: impl Into<String>, s: impl Into<Arc<str>>) -> Self {
        let mut db = Self::default();
        db.set_text(url.into(), s.into());
        db
    }
}

impl salsa::Database for TestDB {}

#[test]
fn test_parse() {
    let db = TestDB::from_str("foo", "select * from table");
    let parsed = db.parse("foo".into());
    expect![[r#"
        SourceFile(
            SourceFile@0..0
              SelectStatement@0..0
                SelectClause@0..0
                  SelectClauseBody@0..0
                    AsteriskExpression@0..0
                FromClause@0..0
                  Identifier@0..0
            ,
        )
    "#]]
    .assert_debug_eq(&parsed.root());
}

#[test]
fn test_parser() {
    let tree = parse("select * from table");
    expect!["(source_file (select_statement (select_clause (select_clause_body (asterisk_expression))) (from_clause (identifier))))"].assert_eq(&tree.root_node().to_sexp());
}
