use psqls_ide::{Rope, TextSize};
use tower_lsp::lsp_types::Position;

use crate::convert::ConvertWith;

#[test]
fn test_convert_size_to_position() {
    let text = "
select * from bar;
insert into foo(x, y) values (1, 2);
    ";
    let rope = Rope::from_str(text);

    let convert = |size: u32| TextSize::from(size).convert_with(&rope);

    assert_eq!(Position::new(0, 0), convert(0));
    assert_eq!(Position::new(1, 0), convert(1));
    assert_eq!(Position::new(1, 1), convert(2));
}
