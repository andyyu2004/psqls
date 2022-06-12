use std::sync::Arc;

use crate::{Change, Ide};

#[macro_export]
macro_rules! url {
    ($url:literal) => {
        ::std::sync::Arc::<str>::from($url)
    };
}

impl Ide {
    pub fn test(url: impl Into<Arc<str>>, sql: &str) -> Self {
        let mut ide = Ide::default();
        ide.apply(Change {
            url: url.into(),
            text: sql.to_owned(),
        });
        ide
    }
}
