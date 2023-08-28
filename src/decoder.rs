use std::io::{Cursor, Read};
use crate::module::*;

pub struct Decoder;

pub enum Component {
    CustomComponent,
    TypeComponent(Vec<FuncType>),
    FuncsComponent(Vec<Function>),
    ImportComponent(Vec<Import>),
    TableComponent(Vec<Table>),
    MemoryComponent(Vec<Mem>),
    GlobalComponent(Vec<Global>),
    ExportComponent(Vec<Export>),
    DataComponent(Vec<Data>),
    ElemComonent(Vec<Elem>),
}

type FunctionSection = Vec<TypeIdx>;
type CodeSection = Vec<(Vec<ValType>, Expr)>;

const MAGIC: [u8; 4] = [0x00, 0x61, 0x73, 0x6d];
const CUSTOM_SECTION_CODE: u8 = 0;
const TYPE_SECTION_CODE: u8 = 1;
const IMPORT_SECTION_CODE: u8 = 2;
const FUNCTION_SECTION_CODE: u8 = 3;
const TABLE_SECTION_CODE: u8 = 4;
const MEMORY_SECTION_CODE: u8 = 5;
const GLOBAL_SECTION_CODE: u8 = 6;
const EXPORT_SECTION_CODE: u8 = 7;
const START_SECTION_CODE: u8 = 8;
const ELEMENT_SECTION_CODE: u8 = 9;
const CODE_SECTION_CODE: u8 = 10;
const DATA_SECTION_CODE: u8 = 11;

impl Decoder {
    pub fn decode(byte_code: &[u8]) -> Result<Module, &'static str> {
        let mut cursor = Cursor::new(byte_code);
        Self::check_magic_number(&mut cursor)?;
        let version = Self::get_version(&mut cursor)?;
        if version != 1 {
            return Err("Version {version} unsupported. Currently there is only version 1 supported");
        };
        let mut byte = [0; 1];
        let mut function_section: Option<FunctionSection>;
        let mut code_section: Option<CodeSection>;
        while let Ok(_) = cursor.read_exact(&mut byte) {
            let section_code = u8::from_le_bytes(byte);
            if section_code == FUNCTION_SECTION_CODE {
                function_section = Some(Self::decode_function_section(&mut cursor)?);
                continue;
            }
            if section_code == CODE_SECTION_CODE {
                code_section = Some(Self::decode_code_section(&mut cursor)?);
                continue;
            }
            let component = Self::decode_section(&mut cursor)?;
            match component {}
        };
        todo!()
    }

    fn check_magic_number(cursor: &mut Cursor<&[u8]>) -> Result<(), &'static str> {
        let mut magic_buffer = [0; 4];
        if let Err(_) = cursor.read_exact(&mut magic_buffer) {
            return Err("Could not read the first 4 bytes of the binary");
        }
        if magic_buffer != MAGIC {
            return Err("Wrong binary magic");
        }
        Ok(())
    }

    fn get_version(cursor: &mut Cursor<&[u8]>) -> Result<u32, &'static str> {
        let mut version_buffer = [0; 4]; 
        if let Err(_) = cursor.read_exact(&mut version_buffer) {
            return Err("Could not read the first 4 bytes of the binary");
        }
        let version = u32::from_le_bytes(version_buffer);
        if version != 1 {
            return Err("Unsupported version {version}. Currently only version 1 is supported")
        }
        Ok(version)
    }

    fn decode_section(cursor: &mut Cursor<&[u8]>) -> Result<Component, &'static str> {
        todo!()
    }

    fn decode_function_section(cursor: &mut Cursor<&[u8]>) -> Result<FunctionSection, &'static str> {
        todo!()
    }

    fn decode_code_section(cursor: &mut Cursor<&[u8]>) -> Result<CodeSection, &'static str> {
        todo!()
    }

    // pub fn parse(byte_code: &[u8]) -> Result<Module, &'static str> {
        // let mut cursor = Cursor::new(byte_code);
        // let mut byte_buffer = [0; 1];
        // let mut int_buffer = [0; 4];
        // if let Err(_) = cursor.read_exact(&mut int_buffer) {
        //     return Err("Could not read the first 4 bytes of the binary");
        // }
        // if i32::from_le_bytes(int_buffer) != i32::from_le_bytes([0x00, 0x61, 0x73, 0x6d]) {
        //     return Err("Wrong magic number");
        // }
        // if let Err(_) = cursor.read_exact(&mut int_buffer) {
        //     return Err("Could not read version of the binary");
        // }
        // let version = i32::from_le_bytes(int_buffer);
        // if let Err(_) = cursor.read_exact(&mut byte_buffer) {
        //     return Ok(Module {
        //         version,
        //         functions: Vec::new(),
        //     });
        // }
        // let current_byte = byte_buffer[0];
        // if current_byte == 0x00 {
        //     Decoder::parse_section_name(&mut cursor);
        // }
        // if current_byte == 0x01 {
        //     let types = Decoder::parse_section_type(&mut cursor);
        // }
        //
        // todo!();
    // }

    // fn parse_section_type(cursor: &mut Cursor<&[u8]>) -> Result<Types, &'static str> {
    //     let mut byte_buffer = [0; 1];
    //     if let Err(_) = cursor.read_exact(&mut byte_buffer) {
    //         return Err("Could not read type section section size");
    //     }
    //     let mut current_byte = byte_buffer[0];
    //     if current_byte != 0x00 {
    //         return Err("Unsupported section size for section type");
    //     }
    //     if let Err(_) = cursor.read_exact(&mut byte_buffer) {
    //         return Err("Could not read type section number of types");
    //     }
    //     let num_types = byte_buffer[0];
    //     let mut types = Vec::new();
    //     for i in num_types {
    //         if let Err(_) = cursor.read_exact(&mut byte_buffer) {
    //             return Err("Could not read type of this type");
    //         } 
    //         let type_type = byte_buffer[0];
    //         if type_type != 0x60 {
    //             return Err("Other types of types then function type are currently not supported");
    //         }
    //         if let Err(_) = cursor.read_exact(&mut byte_buffer) {
    //             return Err("Could not read number of params");
    //         } 
    //         let num_params = byte_buffer[0];
    //         let mut params = Vec::new();
    //         for _ in num_params {
    //             if let Err(_) = cursor.read_exact(&mut byte_buffer) {
    //                 return Err("Could not read parameter type");
    //             }
    //             let param_type = byte_buffer[0];
    //             if param_type == 0x7f {
    //                 params.push(PrimitiveType::I32);
    //             } else {
    //                 return Err("No other primitive type then i32 is currently supported, lol");
    //             }
    //         }
    //         if let Err(_) = cursor.read_exact(&mut byte_buffer) {
    //             return Err("Could not read number of results");
    //         } 
    //         let num_results = byte_buffer[0];
    //         let mut results = Vec::new();
    //         for _ in results {
    //             if let Err(_) = cursor.read_exact(&mut byte_buffer) {
    //                 return Err("Could not read parameter type");
    //             }
    //             let result_type = byte_buffer[0];
    //             if result_type == 0x7f {
    //                 results.push(PrimitiveType::I32);
    //             } else {
    //                 return Err("No other primitive type then i32 is currently supported, lol");
    //             }
    //         }
    //         types.push(Type { params, results });
    //     } 
    //     if let Err(_) = cursor.read_exact(&mut byte_buffer) {
    //         return Err("Could not read FIXUP");
    //     } 
    //     Ok(types)
    // }

//     fn parse_section_name(cursor: &mut Cursor<&[u8]>) -> Result<(), &'static str> {
//         Err("Section name not supported!")
//     }
}
