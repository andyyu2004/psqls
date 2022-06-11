use crate::generated::*;

pub struct ValidationVisitor {}

impl Visitor for ValidationVisitor {
    fn visit_select_statement(&mut self, select_statement: SelectStatement) {}
}
