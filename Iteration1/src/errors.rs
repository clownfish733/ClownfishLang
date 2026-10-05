pub struct CompilerError {
    kind: CompilerErrorKind,
    span: Span,
}

pub struct Span {
    start: usize,
    end: usize,
}

pub enum CompilerErrorKind {
    PRE,
    Lex,
    AST,
    IR,
    ASM,
}

impl CompilerError {
    pub fn message(&self, src: &str) {
        todo!()
    }
}
