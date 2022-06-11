use crate::nodes::*;

// Maybe parse the grammar into a structure more suitable for generating code
// We could then generate the nodes and the visitor etc from this structure
// pub struct Root {
//     name: &'static str,
//     children: Vec<Field>,
// }

// pub struct Field {
//     name: &'static str,
//     cardinality: Cardinality,
//     children: Vec<Field>,
// }

// pub enum Cardinality {
//     One,
//     Many,
// }

// const F: Root = Root {
//     name: "source_file",
//     children: vec![Field {
//         name: "statements",
//         cardinality: Cardinality::Many,
//         children: vec![],
//     }],
// };

pub trait Visitor {
    fn visit_source_file(&mut self, source_file: &SourceFile) {
        source_file
            .statements()
            .for_each(|stmt| self.visit_stmt(stmt));
    }

    fn visit_stmt(&mut self, stmt: &Statement) {}
}
