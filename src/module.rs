pub struct Module {
    pub version: i32,
    pub functions: Vec<Function>,
    // memory: Memory,
    // table: Table,
}

pub struct Function {
    params: Vec<Type>,
    returns: Vec<Type>,
    implementation: Vec<Instr>,
}

pub enum Type {
    I32,
    I64,
    F32,
    F64,
}

pub enum Instr {
    LocalGet(usize),
    I32Add,
}

// pub struct Memory;

// pub struct Table;
