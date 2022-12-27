use std::fmt::{Debug, Formatter};

pub enum StatementType {
    Section,
    Include,
    If,
    Def,
    NewCharMap,
    CharMap,
    SetCharMap,
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
        return "INCLUDE ".to_string() + "\"" + self.path.as_str() + "\"";
    }
}

pub struct SectionStatement {
    pub name: String,
    pub section_type: String,
}

impl Statement for SectionStatement {
    fn my_type(&self) -> StatementType {
        return StatementType::Section;
    }

    fn to_string(&self) -> String {
        return "SECTION ".to_string() + "\"" + self.name.as_str() + "\"";
    }
}

pub struct IfStatement {
}

impl Statement for IfStatement {
    fn my_type(&self) -> StatementType {
        return StatementType::If;
    }

    fn to_string(&self) -> String {
        return "IF ".to_string();
    }
}

pub struct NewCharMapStatement {
    pub name: String
}

impl Statement for NewCharMapStatement {
    fn my_type(&self) -> StatementType {
        return StatementType::NewCharMap;
    }

    fn to_string(&self) -> String {
        return "New Char Map ".to_string() + self.name.as_str();
    }
}

pub struct CharMapStatement {
    pub value: String,
    pub number: i32
}

impl Statement for CharMapStatement {
    fn my_type(&self) -> StatementType {
        return StatementType::NewCharMap;
    }

    fn to_string(&self) -> String {
        return "Char Map \"".to_string() + self.value.as_str() + "\" " + self.number.to_string().as_str();
    }
}

pub struct DefStatement {
    pub name: String,
    pub value: String,
}

impl Statement for DefStatement {
    fn my_type(&self) -> StatementType {
        return StatementType::Def;
    }

    fn to_string(&self) -> String {
        return "DEF \"".to_string() + self.name.as_str() + "\" " + self.value.as_str();
    }
}

pub struct Ast {
    pub statements: Vec<Box<dyn Statement>>
}

impl Debug for Ast {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Ast").expect("Could not write");
        writeln!(f, "Statements: [").expect("Could not write");
        for x in &self.statements {
            writeln!(f, "{}", x.to_string()).expect("Could not write");
        }
        writeln!(f, "]")
    }
}
