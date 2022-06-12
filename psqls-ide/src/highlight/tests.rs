use expect_test::expect;

use crate::{url, Ide};

#[test]
fn test_highlight_keywords() {
    let url = url!("foo.sql");
    let ide = Ide::test(url.clone(), "select * from bar");
    let highlights = ide.snapshot().highlight(url);
    expect![[r#"
        [
            HighlightRange {
                range: 0..6,
                hl: Keyword,
            },
            HighlightRange {
                range: 7..11,
                hl: Keyword,
            },
        ]
    "#]]
    .assert_debug_eq(&highlights);
}
