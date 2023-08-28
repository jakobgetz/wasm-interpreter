pub struct Module {
    pub version: i32,
    pub types: Vec<FuncType>,
    pub funcs: Vec<Function>,
    pub table: Vec<Table>,
    pub memory: Vec<Mem>,
    pub globals: Vec<Global>,
    pub elem: Vec<Elem>,
    pub data: Vec<Data>,
    pub start: Option<Start>,
    pub imports: Vec<Import>,
    pub exports: Vec<Export>,
}

pub struct TypeIdx(u32);
pub struct FuncIdx(u32);
pub struct TableIdx(u32);
pub struct MemIdx(u32);
pub struct GlobalIdx(u32);
pub struct LocalIdx(u32);
pub struct LabelIdx(u32);

pub struct FuncType {
    pub params: Vec<ValType>,
    pub results: Vec<ValType>,
}

pub struct Function {
    typ: TypeIdx,
    locals: Vec<ValType>,
    body: Expr,
}

pub struct Table {
    typ: TableType,
}

pub struct TableType(Limits, ElemType);

pub struct Limits {
    min: u32,
    max: u32,
}

pub enum ElemType {
    FuncRef,
}

pub struct Mem {
    typ: MemType,
}

type MemType = Limits;

pub struct Global {
    typ: GlobalType,
    init: Expr,
}

pub struct GlobalType(Mut, ValType);

pub enum Mut {
    Var,
    Const,
}

pub struct Elem {
    table: TableIdx,
    offset: Expr,
    init: Vec<FuncIdx>,
}

pub struct Data {
    data: MemIdx,
    offset: Expr,
    init: Vec<u8>,
}

pub struct Start {
    func: FuncIdx,
}

pub struct Import {
    module: String,
    name: String,
    desc: ImportDesc,
}

pub enum ImportDesc {
    Func(TypeIdx),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}

pub struct Export {
    name: String,
    desc: ExportDesc,
}

pub enum ExportDesc {
    Func(FuncIdx),
    Table(TableIdx),
    Mem(MemIdx),
    Global(GlobalIdx),
}

pub enum ValType {
    I32,
    I64,
    F32,
    F64,
}

pub struct Expr(Vec<Instr>, End);

pub enum Instr {
    LocalGet(usize),
    I32Add,
}

pub struct End;
