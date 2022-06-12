use expect_test::expect;
use psqls_syn::SyntaxDatabase;

use crate::{url, Ide};

#[test]
fn test_highlight_keywords() {
    let url = url!("foo.sql");
    let ide = Ide::test(url.clone(), "select * from bar");
    let highlights = ide.snapshot().highlight(url);
    expect![[r#"
        []
    "#]]
    .assert_debug_eq(&highlights);
}
