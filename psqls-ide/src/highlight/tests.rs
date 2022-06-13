use expect_test::{expect, Expect};

use crate::{url, Ide};

fn test(sql: &str, expect: Expect) {
    let url = url!("foo.sql");
    let ide = Ide::test(url.clone(), sql);
    let highlights = ide.snapshot().highlight(url);
    expect.assert_debug_eq(&highlights);
}

#[test]
fn test_highlight_keywords() {
    test(
        "select * from bar",
        expect![[r#"
            [
                HighlightRange {
                    range: 0..6,
                    hl: Keyword,
                },
                HighlightRange {
                    range: 9..13,
                    hl: Keyword,
                },
            ]
        "#]],
    )
}
