use crate::codes;
use crate::codes::*;
use crate::module::*;
use std::io::{Cursor, Read};

pub struct Decoder;

type FunctionSection = Vec<TypeIdx>;
type Code = (Vec<ValType>, Expr);
type CodeSection = Vec<Code>;

impl Decoder {
    pub fn decode(byte_code: &[u8]) -> Result<Module, String> {
        let mut cursor = Cursor::new(byte_code);
        Self::check_magic_number(&mut cursor)?;
        let version = Self::get_version(&mut cursor)?;
        if version != 1 {
            return Err(Self::gen_error_msg(
                &cursor,
                format!(
                    "Version {version} unsupported. Currently there is only version 1 supported"
                ),
            ));
        };
        let mut function_section = None;
        let mut code_section = None;
        let mut module = Module::default();
        while let Ok(section_code) = cursor.read_byte() {
            let _section_size = Self::decode_uint(&mut cursor, 32);
            match section_code {
                codes::section::CUSTOM => {
                    return Err(Self::gen_error_msg(
                        &cursor,
                        String::from("Custom are not supported"),
                    ))
                }
                codes::section::TYPE => module.types = Self::decode_type_section(&mut cursor)?,
                codes::section::IMPORT => {
                    module.imports = Self::decode_import_section(&mut cursor)?
                }
                codes::section::CODE => {
                    code_section = Some(Self::decode_code_section(&mut cursor)?)
                }
                codes::section::TABLE => module.table = Self::decode_table_section(&mut cursor)?,
                codes::section::MEMORY => module.memory = Self::decode_memory_section(&mut cursor)?,
                codes::section::GLOBAL => {
                    module.globals = Self::decode_global_section(&mut cursor)?
                }
                codes::section::EXPORT => {
                    module.exports = Self::decode_export_section(&mut cursor)?
                }
                codes::section::START => module.start = Self::decode_start_section(&mut cursor)?,
                codes::section::ELEMENT => module.elem = Self::decode_elem_section(&mut cursor)?,
                codes::section::FUNCTION => {
                    function_section = Some(Self::decode_function_section(&mut cursor)?)
                }
                codes::section::DATA => module.data = Self::decode_data_section(&mut cursor)?,
                _ => {
                    return Err(Self::gen_error_msg(
                        &cursor,
                        String::from("Unsupported section type"),
                    ))
                }
            };
        }
        if let (Some(fs), Some(cs)) = (function_section, code_section) {
            module.funcs = Self::build_functions_component(fs, cs)?;
        }
        Ok(module)
    }

    fn check_magic_number(cursor: &mut Cursor<&[u8]>) -> Result<(), String> {
        let mut magic_buffer = [0; 4];
        cursor.read_exact_custom(&mut magic_buffer)?;
        if magic_buffer != MAGIC {
            return Err(Self::gen_error_msg(
                cursor,
                String::from("Wrong binary magic"),
            ));
        }
        Ok(())
    }

    fn get_version(cursor: &mut Cursor<&[u8]>) -> Result<u32, String> {
        let version = cursor.read_le_i32()?;
        if version != 1 {
            return Err(Self::gen_error_msg(
                cursor,
                String::from(
                    "Unsupported version {version}. Currently only version 1 is supported",
                ),
            ));
        }
        Ok(version)
    }

    fn decode_type_section(cursor: &mut Cursor<&[u8]>) -> Result<TypesComponent, String> {
        Self::process_vector(cursor, |cursor| {
            let typ = cursor.read_byte()?;
            if typ != codes::types::FUNCTION {
                return Err(Self::gen_error_msg(
                    cursor,
                    String::from("Wrong elements stored in section type"),
                ));
            }
            let params = Self::process_vector(cursor, |cursor| Self::decode_val_type(cursor))?;
            let results = Self::process_vector(cursor, |cursor| Self::decode_val_type(cursor))?;
            Ok(FuncType { params, results })
        })
    }

    fn decode_import_section(cursor: &mut Cursor<&[u8]>) -> Result<ImportsComponent, String> {
        Self::process_vector(cursor, |cursor| {
            let module = Self::decode_string(cursor)?;
            let name = Self::decode_string(cursor)?;
            let desc = Self::decode_imp_export_description(cursor)?;
            Ok(Import { module, name, desc })
        })
    }

    fn decode_table_section(cursor: &mut Cursor<&[u8]>) -> Result<TableComponent, String> {
        let table_component = Self::process_vector(cursor, |cursor| {
            Ok(Table {
                typ: Self::decode_table_type(cursor)?,
            })
        })?;
        if table_component.len() > 1 {
            return Err(Self::gen_error_msg(
                cursor,
                String::from("Only one table allowed per module in version 1.0"),
            ));
        }
        Ok(table_component)
    }

    fn decode_memory_section(cursor: &mut Cursor<&[u8]>) -> Result<MemoryComponent, String> {
        let memory_component = Self::process_vector(cursor, |cursor| {
            Ok(Mem {
                typ: Self::decode_mem_type(cursor)?,
            })
        })?;
        if memory_component.len() > 1 {
            return Err(Self::gen_error_msg(
                cursor,
                String::from("Only one memory allowed per module in version 1.0"),
            ));
        }
        Ok(memory_component)
    }

    fn decode_global_section(cursor: &mut Cursor<&[u8]>) -> Result<GlobalsComponent, String> {
        Self::process_vector(cursor, |cursor| {
            let init = Self::decode_expression(cursor)?;
            Ok(Global {
                typ: Self::decode_global_type(cursor)?,
                init,
            })
        })
    }

    fn decode_export_section(cursor: &mut Cursor<&[u8]>) -> Result<ExportsComponent, String> {
        Self::process_vector(cursor, |cursor| {
            let name = Self::decode_string(cursor)?;
            let desc = Self::decode_imp_export_description(cursor)?;
            Ok(Export { name, desc })
        })
    }

    fn decode_imp_export_description(cursor: &mut Cursor<&[u8]>) -> Result<ImpExportDesc, String> {
        match cursor.read_byte()? {
            codes::im_export_desc::TYPE => {
                Ok(ImpExportDesc::Func(TypeIdx(Self::decode_u32(cursor)?)))
            }
            codes::im_export_desc::TABLE => {
                Ok(ImpExportDesc::Table(Self::decode_table_type(cursor)?))
            }
            codes::im_export_desc::MEM => Ok(ImpExportDesc::Mem(Self::decode_mem_type(cursor)?)),
            codes::im_export_desc::GLOBAL => {
                Ok(ImpExportDesc::Global(Self::decode_global_type(cursor)?))
            }
            _ => Err(Self::gen_error_msg(
                cursor,
                String::from("Malicious import description"),
            )),
        }
    }

    fn decode_start_section(cursor: &mut Cursor<&[u8]>) -> Result<StartComponent, String> {
        Ok(Some(Start {
            func: FuncIdx(Self::decode_u32(cursor)?),
        }))
    }

    fn decode_elem_section(cursor: &mut Cursor<&[u8]>) -> Result<ElemComponent, String> {
        Self::process_vector(cursor, |cursor| {
            let table = TableIdx(Self::decode_u32(cursor)?);
            let offset = Self::decode_expression(cursor)?;
            let init =
                Self::process_vector(cursor, |cursor| Ok(FuncIdx(Self::decode_u32(cursor)?)))?;
            Ok(Elem {
                table,
                offset,
                init,
            })
        })
    }

    fn decode_data_section(cursor: &mut Cursor<&[u8]>) -> Result<DataComponent, String> {
        Self::process_vector(cursor, |cursor| {
            let data = MemIdx(Self::decode_u32(cursor)?);
            let offset = Self::decode_expression(cursor)?;
            let init = Self::process_vector(cursor, |cursor| cursor.read_byte())?;
            Ok(Data { data, offset, init })
        })
    }

    fn decode_function_section(cursor: &mut Cursor<&[u8]>) -> Result<FunctionSection, String> {
        let function_section =
            Self::process_vector(cursor, |cursor| Ok(TypeIdx(Self::decode_u32(cursor)?)))?;
        Ok(function_section)
    }

    fn decode_code_section(cursor: &mut Cursor<&[u8]>) -> Result<CodeSection, String> {
        let code_section = Self::process_vector(cursor, |cursor| {
            let _size = Self::decode_u32(cursor)?;
            let code = Self::decode_code(cursor)?;
            Ok(code)
        })?;
        Ok(code_section)
    }

    fn decode_code(cursor: &mut Cursor<&[u8]>) -> Result<Code, String> {
        let locals = Self::process_vector(cursor, |cursor| Self::decode_local(cursor))?
            .into_iter()
            .flat_map(|v| v)
            .collect();
        let expr = Self::decode_expression(cursor)?;
        Ok((locals, expr))
    }

    fn decode_local(cursor: &mut Cursor<&[u8]>) -> Result<Vec<ValType>, String> {
        let n = Self::decode_u32(cursor)?;
        let mut locals = Vec::new();
        if n > 0 {
            let val_type = Self::decode_val_type(cursor)?;
            for _ in 0..n {
                locals.push(val_type.clone())
            }
        }
        Ok(locals)
    }

    fn build_functions_component(
        function_section: FunctionSection,
        code_section: CodeSection,
    ) -> Result<FuncsComponent, String> {
        if function_section.len() != code_section.len() {
            return Err(String::from(
                "Function section and code section dont have the same number of elements",
            ));
        }
        let function_component = function_section
            .into_iter()
            .zip(code_section)
            .map(|(typ, code)| Function {
                typ,
                locals: code.0,
                body: code.1,
            })
            .collect();
        Ok(function_component)
    }

    fn decode_table_type(cursor: &mut Cursor<&[u8]>) -> Result<TableType, String> {
        let elem_type = if cursor.read_byte()? != codes::types::FUNCREF {
            ElemType::FuncRef
        } else {
            return Err(Self::gen_error_msg(
                cursor,
                String::from("This element type is not supported"),
            ));
        };
        let limits = Self::decode_limits(cursor)?;
        Ok(TableType(limits, elem_type))
    }

    fn decode_mem_type(cursor: &mut Cursor<&[u8]>) -> Result<MemType, String> {
        Ok(MemType(Self::decode_limits(cursor)?))
    }

    fn decode_global_type(cursor: &mut Cursor<&[u8]>) -> Result<GlobalType, String> {
        let val_type = Self::decode_val_type(cursor)?;
        let mutablity = match cursor.read_byte()? {
            codes::types::CONST => Mut::Const,
            codes::types::VAR => Mut::Var,
            _ => {
                return Err(Self::gen_error_msg(
                    cursor,
                    String::from("Invalid mutability modifier for global"),
                ))
            }
        };
        Ok(GlobalType(mutablity, val_type))
    }

    fn decode_limits(cursor: &mut Cursor<&[u8]>) -> Result<Limits, String> {
        match cursor.read_byte()? {
            codes::types::LIMIT_NO_MAX => Ok(Limits {
                min: Self::decode_u32(cursor)?,
                max: None,
            }),
            codes::types::LIMIT_MAX => Ok(Limits {
                min: Self::decode_u32(cursor)?,
                max: Some(Self::decode_u32(cursor)?),
            }),
            _ => {
                return Err(Self::gen_error_msg(
                    cursor,
                    String::from("Invalid Limits type"),
                ))
            }
        }
    }

    fn decode_val_type(cursor: &mut Cursor<&[u8]>) -> Result<ValType, String> {
        match cursor.read_byte()? {
            codes::types::I32 => Ok(ValType::I32),
            codes::types::I64 => Ok(ValType::I64),
            codes::types::F32 => Ok(ValType::F32),
            codes::types::F64 => Ok(ValType::F64),
            code => Err(Self::gen_error_msg(
                cursor,
                format!("Could not derive a ValType from code {code:x}"),
            )),
        }
    }

    fn decode_expression(cursor: &mut Cursor<&[u8]>) -> Result<Expr, String> {
        Ok(Expr(Self::decode_instr_until_end(cursor)?, End))
    }

    fn decode_block_type(cursor: &mut Cursor<&[u8]>) -> Result<ResultType, String> {
        match cursor.read_byte()? {
            codes::types::RESULT => Ok(ResultType(None)),
            codes::types::I32 => Ok(ResultType(Some(ValType::I32))),
            codes::types::I64 => Ok(ResultType(Some(ValType::I64))),
            codes::types::F32 => Ok(ResultType(Some(ValType::F32))),
            codes::types::F64 => Ok(ResultType(Some(ValType::F64))),
            _ => Err(Self::gen_error_msg(
                cursor,
                String::from("Invalid blocktype"),
            )),
        }
    }

    fn decode_instr_until_end(cursor: &mut Cursor<&[u8]>) -> Result<Vec<Instr>, String> {
        let mut instructions = Vec::new();
        let mut opcode = cursor.read_byte()?;
        while opcode != codes::instr::END {
            instructions.push(Self::decode_instruction(cursor, opcode)?);
            opcode = cursor.read_byte()?;
        }
        Ok(instructions)
    }

    fn decode_instr_until_else(cursor: &mut Cursor<&[u8]>) -> Result<Vec<Instr>, String> {
        let mut instructions = Vec::new();
        let mut opcode = cursor.read_byte()?;
        while opcode != codes::instr::ELSE {
            instructions.push(Self::decode_instruction(cursor, opcode)?);
            opcode = cursor.read_byte()?;
        }
        Ok(instructions)
    }

    fn decode_instruction(cursor: &mut Cursor<&[u8]>, opcode: u8) -> Result<Instr, String> {
        let instr = match opcode {
            codes::instr::UNREACHABLE => Instr::Unreachable,
            codes::instr::NOP => Instr::Nop,
            codes::instr::BLOCK => Instr::Block(
                Self::decode_block_type(cursor)?,
                Self::decode_instr_until_end(cursor)?,
                End,
            ),
            codes::instr::LOOP => Instr::Loop(
                Self::decode_block_type(cursor)?,
                Self::decode_instr_until_end(cursor)?,
                End,
            ),
            codes::instr::IF => Instr::If(
                Self::decode_block_type(cursor)?,
                Self::decode_instr_until_else(cursor)?,
                Else,
                Self::decode_instr_until_end(cursor)?,
                End,
            ),
            codes::instr::BR => Instr::Br(LabelIdx(Self::decode_u32(cursor)?)),
            codes::instr::BR_IF => Instr::BrIf(LabelIdx(Self::decode_u32(cursor)?)),
            codes::instr::BR_TABLE => Instr::BrTable(
                Self::process_vector(cursor, |cursor| Ok(LabelIdx(Self::decode_u32(cursor)?)))?,
                LabelIdx(Self::decode_u32(cursor)?),
            ),
            codes::instr::RETURN => Instr::Return,
            codes::instr::CALL => Instr::Call(FuncIdx(Self::decode_u32(cursor)?)),
            codes::instr::CALL_INDIRECT => Instr::CallIndirect(TypeIdx(Self::decode_u32(cursor)?)),
            codes::instr::DROP => Instr::Drop,
            codes::instr::SELECT => Instr::Select,
            codes::instr::LOCAL_GET => Instr::LocalGet(LocalIdx(Self::decode_u32(cursor)?)),
            codes::instr::LOCAL_SET => Instr::LocalSet(LocalIdx(Self::decode_u32(cursor)?)),
            codes::instr::LOCAL_TEE => Instr::LocalTee(LocalIdx(Self::decode_u32(cursor)?)),
            codes::instr::GLOBAL_GET => Instr::GlobalGet(GlobalIdx(Self::decode_u32(cursor)?)),
            codes::instr::GLOBAL_SET => Instr::GlobalSet(GlobalIdx(Self::decode_u32(cursor)?)),
            codes::instr::I32_LOAD => Instr::I32Load(MemArg {
                offset: Self::decode_u32(cursor)?,
                align: Self::decode_u32(cursor)?,
            }),
            codes::instr::I64_LOAD => Instr::I64Load(MemArg {
                offset: Self::decode_u32(cursor)?,
                align: Self::decode_u32(cursor)?,
            }),
            codes::instr::F32_LOAD => Instr::F32Load(MemArg {
                offset: Self::decode_u32(cursor)?,
                align: Self::decode_u32(cursor)?,
            }),
            codes::instr::F64_LOAD => Instr::F64Load(MemArg {
                offset: Self::decode_u32(cursor)?,
                align: Self::decode_u32(cursor)?,
            }),
            codes::instr::I32_LOAD8_S => Instr::I32Load8S(MemArg {
                offset: Self::decode_u32(cursor)?,
                align: Self::decode_u32(cursor)?,
            }),
            codes::instr::I32_LOAD8_U => Instr::I32Load8U(MemArg {
                offset: Self::decode_u32(cursor)?,
                align: Self::decode_u32(cursor)?,
            }),
            codes::instr::I32_LOAD16_S => Instr::I32Load16S(MemArg {
                offset: Self::decode_u32(cursor)?,
                align: Self::decode_u32(cursor)?,
            }),
            codes::instr::I32_LOAD16_U => Instr::I32Load16U(MemArg {
                offset: Self::decode_u32(cursor)?,
                align: Self::decode_u32(cursor)?,
            }),
            codes::instr::I64_LOAD8_S => Instr::I64Load8S(MemArg {
                offset: Self::decode_u32(cursor)?,
                align: Self::decode_u32(cursor)?,
            }),
            codes::instr::I64_LOAD8_U => Instr::I64Load8U(MemArg {
                offset: Self::decode_u32(cursor)?,
                align: Self::decode_u32(cursor)?,
            }),
            codes::instr::I64_LOAD16_S => Instr::I64Load16S(MemArg {
                offset: Self::decode_u32(cursor)?,
                align: Self::decode_u32(cursor)?,
            }),
            codes::instr::I64_LOAD16_U => Instr::I64Load16U(MemArg {
                offset: Self::decode_u32(cursor)?,
                align: Self::decode_u32(cursor)?,
            }),
            codes::instr::I64_LOAD32_S => Instr::I64Load32S(MemArg {
                offset: Self::decode_u32(cursor)?,
                align: Self::decode_u32(cursor)?,
            }),
            codes::instr::I64_LOAD32_U => Instr::I64Load32U(MemArg {
                offset: Self::decode_u32(cursor)?,
                align: Self::decode_u32(cursor)?,
            }),
            codes::instr::I32_STORE => Instr::I32Store(MemArg {
                offset: Self::decode_u32(cursor)?,
                align: Self::decode_u32(cursor)?,
            }),
            codes::instr::I64_STORE => Instr::I64Store(MemArg {
                offset: Self::decode_u32(cursor)?,
                align: Self::decode_u32(cursor)?,
            }),
            codes::instr::F32_STORE => Instr::F32Store(MemArg {
                offset: Self::decode_u32(cursor)?,
                align: Self::decode_u32(cursor)?,
            }),
            codes::instr::F64_STORE => Instr::F64Store(MemArg {
                offset: Self::decode_u32(cursor)?,
                align: Self::decode_u32(cursor)?,
            }),
            codes::instr::I32_STORE8 => Instr::I32Store8(MemArg {
                offset: Self::decode_u32(cursor)?,
                align: Self::decode_u32(cursor)?,
            }),
            codes::instr::I32_STORE16 => Instr::I32Store16(MemArg {
                offset: Self::decode_u32(cursor)?,
                align: Self::decode_u32(cursor)?,
            }),
            codes::instr::I64_STORE8 => Instr::I64Store8(MemArg {
                offset: Self::decode_u32(cursor)?,
                align: Self::decode_u32(cursor)?,
            }),
            codes::instr::I64_STORE16 => Instr::I64Store16(MemArg {
                offset: Self::decode_u32(cursor)?,
                align: Self::decode_u32(cursor)?,
            }),
            codes::instr::I64_STORE32 => Instr::I64Store32(MemArg {
                offset: Self::decode_u32(cursor)?,
                align: Self::decode_u32(cursor)?,
            }),
            codes::instr::MEMORY_SIZE => Instr::MemorySize,
            codes::instr::MEMORY_GROW => Instr::MemoryGrow,
            codes::instr::I32_CONST => Instr::I32Const(Self::decode_i32(cursor)?),
            codes::instr::I64_CONST => Instr::I64Const(Self::decode_i64(cursor)?),
            codes::instr::F32_CONST => Instr::F32Const(cursor.read_le_f32()?),
            codes::instr::F64_CONST => Instr::F64Const(cursor.read_le_f64()?),
            codes::instr::I32_EQZ => Instr::I32Eqz,
            codes::instr::I32_EQ => Instr::I32Eq,
            codes::instr::I32_NE => Instr::I32Ne,
            codes::instr::I32_LT_S => Instr::I32LtS,
            codes::instr::I32_LT_U => Instr::I32LtU,
            codes::instr::I32_GT_S => Instr::I32GtS,
            codes::instr::I32_GT_U => Instr::I32GtU,
            codes::instr::I32_LE_S => Instr::I32LeS,
            codes::instr::I32_LE_U => Instr::I32LeU,
            codes::instr::I32_GE_S => Instr::I32GeS,
            codes::instr::I32_GE_U => Instr::I32GeU,
            codes::instr::I64_EQZ => Instr::I64Eqz,
            codes::instr::I64_EQ => Instr::I64Eq,
            codes::instr::I64_NE => Instr::I64Ne,
            codes::instr::I64_LT_S => Instr::I64LtS,
            codes::instr::I64_LT_U => Instr::I64LtU,
            codes::instr::I64_GT_S => Instr::I64GtS,
            codes::instr::I64_GT_U => Instr::I64GtU,
            codes::instr::I64_LE_S => Instr::I64LeS,
            codes::instr::I64_LE_U => Instr::I64LeU,
            codes::instr::I64_GE_S => Instr::I64GeS,
            codes::instr::I64_GE_U => Instr::I64GeU,
            codes::instr::F32_EQ => Instr::F32Eq,
            codes::instr::F32_NE => Instr::F32Ne,
            codes::instr::F32_LT => Instr::F32Lt,
            codes::instr::F32_GT => Instr::F32Gt,
            codes::instr::F32_LE => Instr::F32Le,
            codes::instr::F32_GE => Instr::F32Ge,
            codes::instr::F64_EQ => Instr::F64Eq,
            codes::instr::F64_NE => Instr::F64Ne,
            codes::instr::F64_LT => Instr::F64Lt,
            codes::instr::F64_GT => Instr::F64Gt,
            codes::instr::F64_LE => Instr::F64Le,
            codes::instr::F64_GE => Instr::F64Ge,
            codes::instr::I32_CLZ => Instr::I32Clz,
            codes::instr::I32_CTZ => Instr::I32Ctz,
            codes::instr::I32_POPCNT => Instr::I32Popcnt,
            codes::instr::I32_ADD => Instr::I32Add,
            codes::instr::I32_SUB => Instr::I32Sub,
            codes::instr::I32_MUL => Instr::I32Mul,
            codes::instr::I32_DIV_S => Instr::I32DivS,
            codes::instr::I32_DIV_U => Instr::I32DivU,
            codes::instr::I32_REM_S => Instr::I32RemS,
            codes::instr::I32_REM_U => Instr::I32RemU,
            codes::instr::I32_AND => Instr::I32And,
            codes::instr::I32_OR => Instr::I32Or,
            codes::instr::I32_XOR => Instr::I32Xor,
            codes::instr::I32_SHL => Instr::I32Shl,
            codes::instr::I32_SHR_S => Instr::I32ShrS,
            codes::instr::I32_SHR_U => Instr::I32ShrU,
            codes::instr::I32_ROTL => Instr::I32Rotl,
            codes::instr::I32_ROTR => Instr::I32Rotr,
            codes::instr::I64_CLZ => Instr::I64Clz,
            codes::instr::I64_CTZ => Instr::I64Ctz,
            codes::instr::I64_POPCNT => Instr::I64Popcnt,
            codes::instr::I64_ADD => Instr::I64Add,
            codes::instr::I64_SUB => Instr::I64Sub,
            codes::instr::I64_MUL => Instr::I64Mul,
            codes::instr::I64_DIV_S => Instr::I64DivS,
            codes::instr::I64_DIV_U => Instr::I64DivU,
            codes::instr::I64_REM_S => Instr::I64RemS,
            codes::instr::I64_REM_U => Instr::I64RemU,
            codes::instr::I64_AND => Instr::I64And,
            codes::instr::I64_OR => Instr::I64Or,
            codes::instr::I64_XOR => Instr::I64Xor,
            codes::instr::I64_SHL => Instr::I64Shl,
            codes::instr::I64_SHR_S => Instr::I64ShrS,
            codes::instr::I64_SHR_U => Instr::I64ShrU,
            codes::instr::I64_ROTL => Instr::I64Rotl,
            codes::instr::I64_ROTR => Instr::I64Rotr,
            codes::instr::F32_ABS => Instr::F32Abs,
            codes::instr::F32_NEG => Instr::F32Neg,
            codes::instr::F32_CEIL => Instr::F32Ceil,
            codes::instr::F32_FLOOR => Instr::F32Floor,
            codes::instr::F32_TRUNC => Instr::F32Trunc,
            codes::instr::F32_NEAREST => Instr::F32Nearest,
            codes::instr::F32_SQRT => Instr::F32Sqrt,
            codes::instr::F32_ADD => Instr::F32Add,
            codes::instr::F32_SUB => Instr::F32Sub,
            codes::instr::F32_MUL => Instr::F32Mul,
            codes::instr::F32_DIV => Instr::F32Div,
            codes::instr::F32_MIN => Instr::F32Min,
            codes::instr::F32_MAX => Instr::F32Max,
            codes::instr::F32_COPYSIGN => Instr::F32Copysign,
            codes::instr::F64_ABS => Instr::F64Abs,
            codes::instr::F64_NEG => Instr::F64Neg,
            codes::instr::F64_CEIL => Instr::F64Ceil,
            codes::instr::F64_FLOOR => Instr::F64Floor,
            codes::instr::F64_TRUNC => Instr::F64Trunc,
            codes::instr::F64_NEAREST => Instr::F64Nearest,
            codes::instr::F64_SQRT => Instr::F64Sqrt,
            codes::instr::F64_ADD => Instr::F64Add,
            codes::instr::F64_SUB => Instr::F64Sub,
            codes::instr::F64_MUL => Instr::F64Mul,
            codes::instr::F64_DIV => Instr::F64Div,
            codes::instr::F64_MIN => Instr::F64Min,
            codes::instr::F64_MAX => Instr::F64Max,
            codes::instr::F64_COPYSIGN => Instr::F64Copysign,
            codes::instr::I32_WRAP_I64 => Instr::I32WrapI64,
            codes::instr::I32_TRUNC_F32_S => Instr::I32TruncF32S,
            codes::instr::I32_TRUNC_F32_U => Instr::I32TruncF32U,
            codes::instr::I32_TRUNC_F64_S => Instr::I32TruncF64S,
            codes::instr::I32_TRUNC_F64_U => Instr::I32TruncF64U,
            codes::instr::I64_EXTEND_I32_S => Instr::I64ExtendI32S,
            codes::instr::I64_EXTEND_I32_U => Instr::I64ExtendI32U,
            codes::instr::I64_TRUNC_F32_S => Instr::I64TruncF32S,
            codes::instr::I64_TRUNC_F32_U => Instr::I64TruncF32U,
            codes::instr::I64_TRUNC_F64_S => Instr::I64TruncF64S,
            codes::instr::I64_TRUNC_F64_U => Instr::I64TruncF64U,
            codes::instr::F32_DEMOTE_F64_S => Instr::F32DemoteF64,
            codes::instr::F32_CONVERT_I32_S => Instr::F32ConvertI32S,
            codes::instr::F32_CONVERT_I32_U => Instr::F32ConvertI32U,
            codes::instr::F32_CONVERT_I64_S => Instr::F32ConvertI64S,
            codes::instr::F32_CONVERT_I64_U => Instr::F32ConvertI64U,
            codes::instr::F64_CONVERT_I32_S => Instr::F64ConvertI32S,
            codes::instr::F64_CONVERT_I32_U => Instr::F64ConvertI32U,
            codes::instr::F64_CONVERT_I64_S => Instr::F64ConvertI64S,
            codes::instr::F64_CONVERT_I64_U => Instr::F64ConvertI64U,
            codes::instr::F64_PROMOTE_F32 => Instr::F64PromoteF32,
            codes::instr::F32_REINTERPRET_I32 => Instr::F32ReinterpretI32,
            codes::instr::F64_REINTERPRET_I64 => Instr::F64ReinterpretI64,
            codes::instr::I32_REINTERPRET_F32 => Instr::I32ReinterpretF32,
            codes::instr::I64_REINTERPRET_F64 => Instr::I64ReinterpretF64,
            _ => {
                return Err(Self::gen_error_msg(
                    cursor,
                    String::from("The instruction with opcode {opcode} is currently not supported"),
                ))
            }
        };
        Ok(instr)
    }

    fn decode_u32(cursor: &mut Cursor<&[u8]>) -> Result<u32, String> {
        Self::decode_uint(cursor, 32).map(|x| x as u32)
    }

    fn decode_uint(cursor: &mut Cursor<&[u8]>, size: u32) -> Result<u64, String> {
        let n: u64 = cursor.read_byte()?.into();
        if n < 128 && n < 2_u64.pow(size) as u64 {
            Ok(n)
        } else if n >= 128 && size > 7 {
            Ok(128 * Self::decode_uint(cursor, size - 7)? + (n - 128))
        } else {
            Err(Self::gen_error_msg(
                cursor,
                String::from("Error decoding uint"),
            ))
        }
    }

    fn decode_i32(cursor: &mut Cursor<&[u8]>) -> Result<i32, String> {
        Self::decode_int(cursor, 32).map(|x| x as i32)
    }

    fn decode_i64(cursor: &mut Cursor<&[u8]>) -> Result<i64, String> {
        Self::decode_int(cursor, 64).map(|x| x as i64)
    }

    fn decode_int(cursor: &mut Cursor<&[u8]>, size: u32) -> Result<i64, String> {
        let n: i64 = cursor.read_byte()?.into();
        if n < 64 && n < 2_u64.pow(size - 1) as i64 {
            Ok(n)
        } else if 64 <= n && n <= 128 && n >= 128 - 2_u64.pow(size - 1) as i64 {
            Ok(n - 128)
        } else if n >= 128 && size > 7 {
            Ok(128 * Self::decode_int(cursor, size - 7)? + (n - 128))
        } else {
            Err(Self::gen_error_msg(
                cursor,
                String::from("Error decoding int"),
            ))
        }
    }

    fn process_vector<F, R>(cursor: &mut Cursor<&[u8]>, f: F) -> Result<Vec<R>, String>
    where
        F: Fn(&mut Cursor<&[u8]>) -> Result<R, String>,
    {
        let length = Self::decode_u32(cursor)?;
        let mut vec = Vec::default();
        for _ in 0..length {
            match f(cursor) {
                Ok(r) => vec.push(r),
                Err(err) => {
                    return Err(Self::gen_error_msg(
                        cursor,
                        format!("Error processing Vector: {err}"),
                    ))
                }
            }
        }
        Ok(vec)
    }

    fn decode_string(cursor: &mut Cursor<&[u8]>) -> Result<String, String> {
        let length = Self::decode_u32(cursor)?;
        let mut string = String::default();
        for _ in 0..length {
            string.push(cursor.read_byte()? as char);
        }
        Ok(string)
    }

    fn gen_error_msg(cursor: &Cursor<&[u8]>, msg: String) -> String {
        format!("Byte address: {:x}, {}", cursor.position() - 1, msg)
    }
}

trait ReadExt: Read {
    fn read_exact_custom(&mut self, buf: &mut [u8]) -> Result<(), String>;
    fn read_byte(&mut self) -> Result<u8, String>;
    fn read_le_i32(&mut self) -> Result<u32, String>;
    fn read_le_f32(&mut self) -> Result<f32, String>;
    fn read_le_f64(&mut self) -> Result<f64, String>;
}
impl ReadExt for Cursor<&[u8]> {
    fn read_exact_custom(&mut self, buf: &mut [u8]) -> Result<(), String> {
        self.read_exact(buf)
            .map_err(|_| String::from("Buffer read error"))
    }

    fn read_byte(&mut self) -> Result<u8, String> {
        let mut byte_buf = [0; 1];
        let _ = self.read_exact_custom(&mut byte_buf)?;
        Ok(u8::from_le_bytes(byte_buf))
    }

    fn read_le_i32(&mut self) -> Result<u32, String> {
        let mut int_buf = [0; 4];
        let _ = self.read_exact_custom(&mut int_buf)?;
        Ok(u32::from_le_bytes(int_buf))
    }

    fn read_le_f32(&mut self) -> Result<f32, String> {
        let mut float_buf = [0; 4];
        let _ = self.read_exact_custom(&mut float_buf)?;
        Ok(f32::from_le_bytes(float_buf))
    }

    fn read_le_f64(&mut self) -> Result<f64, String> {
        let mut float_buf = [0; 8];
        let _ = self.read_exact_custom(&mut float_buf)?;
        Ok(f64::from_le_bytes(float_buf))
    }
}
