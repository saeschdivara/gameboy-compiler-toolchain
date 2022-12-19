use std::fmt::{Debug, Formatter};

pub enum StatementType {
    Section,
    Include,
}

pub trait Statement {
    fn my_type(&self) -> StatementType;
    fn to_string(&self) -> String;
}

pub struct IncludeStatement {
    pub path: String
}

impl Statement for IncludeStatement {
    fn my_type(&self) -> StatementType {
        return StatementType::Include;
    }

    fn to_string(&self) -> String {
        return "INCLUDE ".to_string() + &*self.path;
    }
}

pub struct Ast {
    pub statements: Vec<Box<dyn Statement>>
}

impl Debug for Ast {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Ast");
        writeln!(f, "Statements: [");
        for x in &self.statements {
            writeln!(f, "{}", x.to_string());
        }
        writeln!(f, "]")
    }
}
