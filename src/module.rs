#[derive(Default, Debug)]
pub struct Module {
    pub version: i32,
    pub types: TypesComponent,
    pub funcs: FuncsComponent,
    pub table: TableComponent,
    pub memory: MemoryComponent,
    pub globals: GlobalsComponent,
    pub elem: ElemComponent,
    pub data: DataComponent,
    pub start: StartComponent,
    pub imports: ImportsComponent,
    pub exports: ExportsComponent,
}

pub type TypesComponent = Vec<FuncType>;
pub type FuncsComponent = Vec<Function>;
pub type TableComponent = Vec<Table>;
pub type MemoryComponent = Vec<Mem>;
pub type GlobalsComponent = Vec<Global>;
pub type ElemComponent = Vec<Elem>;
pub type DataComponent = Vec<Data>;
pub type StartComponent = Option<Start>;
pub type ImportsComponent = Vec<Import>;
pub type ExportsComponent = Vec<Export>;

#[derive(Debug)]
pub struct TypeIdx(pub u32);
#[derive(Debug)]
pub struct FuncIdx(pub u32);
#[derive(Debug)]
pub struct TableIdx(pub u32);
#[derive(Debug)]
pub struct MemIdx(pub u32);
#[derive(Debug)]
pub struct GlobalIdx(pub u32);
#[derive(Debug)]
pub struct LocalIdx(pub u32);
#[derive(Debug)]
pub struct LabelIdx(pub  u32);

#[derive(Debug)]
pub struct FuncType {
    pub params: Vec<ValType>,
    pub results: Vec<ValType>,
}

#[derive(Debug)]
pub struct Function {
    typ: TypeIdx,
    locals: Vec<ValType>,
    body: Expr,
}

#[derive(Debug)]
pub struct Table {
    pub typ: TableType,
}

#[derive(Debug)]
pub struct TableType(pub Limits, pub ElemType);

#[derive(Debug)]
pub struct Limits {
    min: u32,
    max: u32,
}

#[derive(Debug)]
pub enum ElemType {
    FuncRef,
}

#[derive(Debug)]
pub struct Mem {
    pub typ: MemType,
}

#[derive(Debug)]
pub struct MemType(pub Limits);

#[derive(Debug)]
pub struct Global {
    pub typ: GlobalType,
    pub init: Expr,
}

#[derive(Debug)]
pub struct GlobalType(pub Mut, pub ValType);

#[derive(Debug)]
pub enum Mut {
    Var,
    Const,
}

#[derive(Debug)]
pub struct Elem {
    table: TableIdx,
    offset: Expr,
    init: Vec<FuncIdx>,
}

#[derive(Debug)]
pub struct Data {
    data: MemIdx,
    offset: Expr,
    init: Vec<u8>,
}

#[derive(Debug)]
pub struct Start {
    func: FuncIdx,
}

#[derive(Debug)]
pub struct Import {
    pub module: String,
    pub name: String,
    pub desc: ImportDesc,
}

#[derive(Debug)]
pub enum ImportDesc {
    Func(TypeIdx),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}

#[derive(Debug)]
pub struct Export {
    name: String,
    desc: ExportDesc,
}

#[derive(Debug)]
pub enum ExportDesc {
    Func(FuncIdx),
    Table(TableIdx),
    Mem(MemIdx),
    Global(GlobalIdx),
}

#[derive(Debug)]
pub enum ValType {
    I32,
    I64,
    F32,
    F64,
}

#[derive(Debug)]
pub struct Expr(pub Vec<Instr>, pub End);

#[derive(Debug)]
pub enum Instr {
    Unreachable,
    Nop,
    Block,
    Loop,
    If,
    Else,
    End,
    Br(LabelIdx),
    BrIf(LabelIdx),
    BrTable(Vec<LabelIdx>),
    Return,
    Call(FuncIdx),
    CallIndirect(TypeIdx),
    Drop,
    Select,
    LocalGet(LocalIdx),
    LocalSet(LocalIdx),
    LocalTee(LocalIdx),
    GlobalGet(GlobalIdx),
    GlobalSet(GlobalIdx),
    I32Add,
}

#[derive(Debug)]
pub struct End;
