use std::io::{Cursor, Read};
use crate::codes;
use crate::module::*;
use crate::codes::*;

pub struct Decoder;

type FunctionSection = Vec<TypeIdx>;
type CodeSection = Vec<(Vec<ValType>, Expr)>;

impl Decoder {
    pub fn decode(byte_code: &[u8]) -> Result<Module, &'static str> {
        let mut cursor = Cursor::new(byte_code);
        Self::check_magic_number(&mut cursor)?;
        let version = Self::get_version(&mut cursor)?;
        if version != 1 {
            return Err("Version {version} unsupported. Currently there is only version 1 supported");
        };
        let mut function_section = None;
        let mut code_section = None;
        let mut module = Module::default();
        while let Ok(section_code) = cursor.read_byte() {
            let _section_size = Self::decode_uint(&mut cursor, 32);
            match section_code {
                codes::section::CUSTOM => return Err("Custom are not supported"),
                codes::section::TYPE => module.types = Self::decode_type_section(&mut cursor)?,
                codes::section::IMPORT => module.imports = Self::decode_import_section(&mut cursor)?,
                codes::section::CODE => code_section = Some(Self::decode_code_section(&mut cursor)?),
                codes::section::TABLE => module.table = Self::decode_table_section(&mut cursor)?,
                codes::section::MEMORY => module.memory = Self::decode_memory_section(&mut cursor)?,
                codes::section::GLOBAL => module.globals = Self::decode_global_section(&mut cursor)?,
                codes::section::EXPORT => module.exports = Self::decode_export_section(&mut cursor)?,
                codes::section::START => module.start = Self::decode_start_section(&mut cursor)?,
                codes::section::ELEMENT => module.elem = Self::decode_elem_section(&mut cursor)?,
                codes::section::FUNCTION => function_section = Some(Self::decode_function_section(&mut cursor)?),
                codes::section::DATA => module.data = Self::decode_data_section(&mut cursor)?,
                _ => return Err("Unsupported section type")
            };
        };
        if let (Some(fs), Some(cs)) =
            (function_section, code_section) {
            module.funcs = Self::build_functions_component(fs, cs);
        }
        Ok(module)
    }

    fn check_magic_number(cursor: &mut Cursor<&[u8]>) -> Result<(), &'static str> {
        let mut magic_buffer = [0; 4];
        cursor.read_exact_custom(&mut magic_buffer)?;
        if magic_buffer != MAGIC {
            return Err("Wrong binary magic");
        }
        Ok(())
    }

    fn get_version(cursor: &mut Cursor<&[u8]>) -> Result<u32, &'static str> {
        let version = cursor.read_le_int32()?;
        if version != 1 {
            return Err("Unsupported version {version}. Currently only version 1 is supported")
        }
        Ok(version)
    }

    fn decode_type_section(cursor: &mut Cursor<&[u8]>) -> Result<TypesComponent, &'static str> {
        let types_component = Self::process_vector(cursor, |cursor| {
            let typ = cursor.read_byte()?;
            if typ != codes::types::FUNCTION {
                return Err("Wrong elements stored in section type");
            }
            let params = Self::process_vector(cursor, |cursor| { Self::decode_val_type(cursor) })?;
            let results = Self::process_vector(cursor, |cursor| { Self::decode_val_type(cursor) })?;
            Ok(FuncType { params, results })
        })?;
        Ok(types_component)
    }

    fn decode_import_section(cursor: &mut Cursor<&[u8]>) -> Result<ImportsComponent, &'static str> {
        let imports_component = Self::process_vector(cursor, |cursor| {
            let module = Self::decode_string(cursor)?;
            let name = Self::decode_string(cursor)?;
            let desc = match cursor.read_byte()? {
                codes::import_desc::TYPE => ImportDesc::Func(TypeIdx(Self::decode_uint(cursor, 32)?)),
                codes::import_desc::TABLE => ImportDesc::Table(Self::decode_table_type(cursor)?),
                codes::import_desc::MEM => ImportDesc::Mem(Self::decode_mem_type(cursor)?),
                codes::import_desc::GLOBAL => ImportDesc::Global(Self::decode_global_type(cursor)?),
                _ => return Err("Malicious import description")
            };
            Ok(Import { module, name, desc })
        })?;
        Ok(imports_component)
    }

    fn decode_table_section(cursor: &mut Cursor<&[u8]>) -> Result<TableComponent, &'static str> {
        let table_component = Self::process_vector(cursor, |cursor| {
            Ok(Table { typ: Self::decode_table_type(cursor)? })
        })?;
        if table_component.len() > 1 {
            return Err("Only one table allowed per module in version 1.0");
        }
        Ok(table_component)
    }

    fn decode_memory_section(cursor: &mut Cursor<&[u8]>) -> Result<MemoryComponent, &'static str> {
        let memory_component = Self::process_vector(cursor, |cursor| {
            Ok(Mem{ typ: Self::decode_mem_type(cursor)? })
        })?;
        if memory_component.len() > 1 {
            return Err("Only one memory allowed per module in version 1.0");
        }
        Ok(memory_component)
    }

    fn decode_global_section(cursor: &mut Cursor<&[u8]>) -> Result<GlobalsComponent, &'static str> {
        let global_component = Self::process_vector(cursor, |cursor| {
            let init = Self::decode_expression(cursor)?;
            Ok(Global { typ: Self::decode_global_type(cursor)?, init })
        })?;
        Ok(global_component)
    }

    fn decode_export_section(cursor: &mut Cursor<&[u8]>) -> Result<ExportsComponent, &'static str> {
        todo!()
    }

    fn decode_start_section(cursor: &mut Cursor<&[u8]>) -> Result<StartComponent, &'static str> {
        todo!()
    }

    fn decode_elem_section(cursor: &mut Cursor<&[u8]>) -> Result<ElemComponent, &'static str> {
        todo!()
    }

    fn decode_data_section(cursor: &mut Cursor<&[u8]>) -> Result<DataComponent, &'static str> {
        todo!()
    }

    fn decode_function_section(cursor: &mut Cursor<&[u8]>) -> Result<FunctionSection, &'static str> {
        todo!()
    }

    fn decode_code_section(cursor: &mut Cursor<&[u8]>) -> Result<CodeSection, &'static str> {
        todo!()
    }

    fn build_functions_component(function_section: FunctionSection, code_section: CodeSection) -> FuncsComponent {
        todo!()
    }

    fn decode_table_type(cursor: &mut Cursor<&[u8]>) -> Result<TableType, &'static str> {
        let elem_type = if cursor.read_byte()? != codes::types::FUNCREF {
            ElemType::FuncRef
        } else {
            return Err("This element type is not supported");
        };
        let limits = Self::decode_limits(cursor)?;
        Ok(TableType(limits, elem_type))
    }

    fn decode_mem_type(cursor: &mut Cursor<&[u8]>) -> Result<MemType, &'static str> {
        Ok(MemType(Self::decode_limits(cursor)?))
    }

    fn decode_global_type(cursor: &mut Cursor<&[u8]>) -> Result<GlobalType, &'static str> {
        let val_type = Self::decode_val_type(cursor)?;
        let mutablity = match cursor.read_byte()? {
            codes::types::CONST => Mut::Const,
            codes::types::VAR => Mut::Var,
            _ => return Err("Invalid mutability modifier for global")
        };
        Ok(GlobalType(mutablity, val_type))
    }

    fn decode_limits(cursor: &mut Cursor<&[u8]>) -> Result<Limits, &'static str> {
        todo!()
    }

    fn decode_val_type(cursor: &mut Cursor<&[u8]>) -> Result<ValType, &'static str> {
        match cursor.read_byte()? {
            codes::types::I32 => Ok(ValType::I32),
            codes::types::I64 => Ok(ValType::I64),
            codes::types::F32 => Ok(ValType::F32),
            codes::types::F64 => Ok(ValType::F64),
            _ => Err("Could not derive a ValType from code {code}")
        }
    }

    fn decode_expression(cursor: &mut Cursor<&[u8]>) -> Result<Expr, &'static str> {
        let mut instructions = Vec::new();
        let opcode = cursor.read_byte()?;
        while opcode != codes::expr::END {
            instructions.push(Self::decode_instruction(cursor, opcode)?);
        }
        Ok(Expr(instructions, End))
    }

    fn decode_instruction(cursor: &mut Cursor<&[u8]>, opcode: u8) -> Result<Instr, &'static str> {
        let instr = match opcode {
            codes::instr::UNREACHABLE => Instr::Unreachable,
            codes::instr::NOP => Instr::Nop,
            codes::instr::LOCAL_GET => Instr::LocalGet(LocalIdx(Self::decode_uint(cursor, 32)?)),
            _ => return Err("The instruction with opcode {opcode} is currently not supported")
        };
        Ok(instr)
    }

    fn decode_uint(cursor: &mut Cursor<&[u8]>, size: u32) -> Result<u32, &'static str> {
        let n: u32 = cursor.read_byte()?.into();
        if n < 128 && n < 2_u32.pow(size) {
            Ok(n)
        } else {
            Ok(128 * Self::decode_uint(cursor, size - 7)? + (n - 128))
        }
    }

    fn process_vector<F, R>(cursor: &mut Cursor<&[u8]>, f: F) -> Result<Vec<R>, &'static str>
        where
            F: Fn(&mut Cursor<&[u8]>) -> Result<R, &'static str>,
    {
        let length = Self::decode_uint(cursor, 32)?;
        let mut vec = Vec::default();
        for _ in 0..length {
            match f(cursor) {
                Ok(r) => vec.push(r),
                Err(err) => return Err("Error processing Vector: {err}"),
            }
        }
        Ok(vec)
    }

    fn decode_string(cursor: &mut Cursor<&[u8]>) -> Result<String, &'static str> {
        let length = Self::decode_uint(cursor, 32)?;
        let mut string = String::default();
        for _ in 0..length {
            string.push(cursor.read_byte()? as char); 
        }
        Ok(string)
    }
}

trait ReadExt: Read {
    fn read_exact_custom(&mut self, buf: &mut [u8]) -> Result<(), &'static str>;
    fn read_byte(&mut self) -> Result<u8, &'static str>;
    fn read_le_int32(&mut self) -> Result<u32, &'static str>;
}
impl ReadExt for Cursor<&[u8]> {
    fn read_exact_custom(&mut self, buf: &mut [u8]) -> Result<(), &'static str> {
        self.read_exact(buf).map_err(|_| "Buffer read error")
    }

    fn read_byte(&mut self) -> Result<u8, &'static str> {
        let mut byte_buf = [0; 1];
        let _ = self.read_exact_custom(&mut byte_buf)?;
        Ok(u8::from_le_bytes(byte_buf))
    }

    fn read_le_int32(&mut self) -> Result<u32, &'static str> {
        let mut int_buf = [0; 4];
        let _ = self.read_exact_custom(&mut int_buf)?;
        Ok(u32::from_le_bytes(int_buf))
    }
}
