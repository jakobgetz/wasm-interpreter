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
pub struct LabelIdx(pub u32);

#[derive(Debug)]
pub struct FuncType {
    pub params: Vec<ValType>,
    pub results: Vec<ValType>,
}

#[derive(Debug)]
pub struct Function {
    pub typ: TypeIdx,
    pub locals: Vec<ValType>,
    pub body: Expr,
}

#[derive(Debug)]
pub struct Table {
    pub typ: TableType,
}

#[derive(Debug)]
pub struct TableType(pub Limits, pub ElemType);

#[derive(Debug)]
pub struct Limits {
    pub min: u32,
    pub max: Option<u32>,
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
    pub table: TableIdx,
    pub offset: Expr,
    pub init: Vec<FuncIdx>,
}

#[derive(Debug)]
pub struct Data {
    pub data: MemIdx,
    pub offset: Expr,
    pub init: Vec<u8>,
}

#[derive(Debug)]
pub struct Start {
    pub func: FuncIdx,
}

#[derive(Debug)]
pub struct Import {
    pub module: String,
    pub name: String,
    pub desc: ImpExportDesc,
}

#[derive(Debug)]
pub enum ImpExportDesc {
    Func(TypeIdx),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}

#[derive(Debug)]
pub struct Export {
    pub name: String,
    pub desc: ImpExportDesc,
}

#[derive(Debug, Clone)]
pub enum ValType {
    I32,
    I64,
    F32,
    F64,
}

#[derive(Debug)]
pub struct ResultType(pub Option<ValType>);

#[derive(Debug)]
pub struct MemArg {
    pub offset: u32,
    pub align: u32,
}

#[derive(Debug)]
pub struct Expr(pub Vec<Instr>, pub End);

#[derive(Debug)]
pub enum Instr {
    Unreachable,
    Nop,
    Block(ResultType, Vec<Instr>, End),
    Loop(ResultType, Vec<Instr>, End),
    If(ResultType, Vec<Instr>, Else, Vec<Instr>, End),
    Br(LabelIdx),
    BrIf(LabelIdx),
    BrTable(Vec<LabelIdx>, LabelIdx),
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
    I32Load(MemArg),
    I64Load(MemArg),
    F32Load(MemArg),
    F64Load(MemArg),
    I32Load8S(MemArg),
    I32Load8U(MemArg),
    I32Load16S(MemArg),
    I32Load16U(MemArg),
    I64Load8S(MemArg),
    I64Load8U(MemArg),
    I64Load16S(MemArg),
    I64Load16U(MemArg),
    I64Load32S(MemArg),
    I64Load32U(MemArg),
    I32Store(MemArg),
    I64Store(MemArg),
    F32Store(MemArg),
    F64Store(MemArg),
    I32Store8(MemArg),
    I32Store16(MemArg),
    I64Store8(MemArg),
    I64Store16(MemArg),
    I64Store32(MemArg),
    MemorySize,
    MemoryGrow,
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),
    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,
    I64Eqz,
    I64Eq,
    I64Ne,
    I64LtS,
    I64LtU,
    I64GtS,
    I64GtU,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,
    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,
    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,
    I32Clz,
    I32Ctz,
    I32Popcnt,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,
    I64Clz,
    I64Ctz,
    I64Popcnt,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,
    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32Copysign,
    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64Copysign,
    I32WrapI64,
    I32TruncF32S,
    I32TruncF32U,
    I32TruncF64S,
    I32TruncF64U,
    I64ExtendI32S,
    I64ExtendI32U,
    I64TruncF32S,
    I64TruncF32U,
    I64TruncF64S,
    I64TruncF64U,
    F32ConvertI32S,
    F32ConvertI32U,
    F32ConvertI64S,
    F32ConvertI64U,
    F32DemoteF64,
    F64ConvertI32S,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI64U,
    F64PromoteF32,
    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,
}

#[derive(Debug)]
pub struct Else;

#[derive(Debug)]
pub struct End;
