pub struct Program {
    pub functions: Vec<Function>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }
}

pub enum Type {
    Int,
    Float,
    Bool,
}

pub enum Value {
    Int(i32),
    Float(f64),
    Bool(bool),
}

pub struct Variable {
    pub name: String,
    pub t: Type,
}

impl Variable {
    pub fn new(name: String, t: Type) -> Self {
        Self {
            name,
            t,
        }
    }
}

pub struct Scope {
    pub variables: Vec<Variable>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
        }
    }
}

pub enum Operator {
    Inc,
    Dec,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Sqrt,
    Abs,
    GR,
    LSEQ,
    EQ,
    AND,
    OR,
    XOR,
    NOT,
    BitL,
    BitR,
}

pub enum FunctionOperation {
    Assign(String, Term),
    Return(Term),
    If(Term, i64),
    While(Term, i64),
}

pub struct Term {
    pub v_type: Type,
    pub tokens: Vec<TermToken>,
}

impl Term {
    pub fn new(v_type: Type, tokens: Vec<TermToken>) -> Self {
        Self {
            v_type,
            tokens,
        }
    }
}

pub enum TermToken {
    C(Value),
    V(String),
    O(Operator),
    P(i64),
}

pub struct Function {
    pub scope: Scope,
    pub procedures: Vec<Vec<FunctionOperation>>,
}

impl Function {
    pub fn new() -> Self {
        Self {
            scope: Scope::new(),
            procedures: Vec::new(),
        }
    }
}