use ::runtime::procedure::Procedure;

pub struct Function {
    pub procedure: Procedure,
    pub argsz: i32,
    pub rsz: i32,
}