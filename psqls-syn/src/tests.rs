use expect_test::expect;

use crate::parse;

#[test]
fn test_parse() {
    let tree = parse("select * from table");
    expect!["(source_file (select_statement (select_clause (select_clause_body (asterisk_expression))) (from_clause (identifier))))"].assert_eq(&tree.root_node().to_sexp());
}
