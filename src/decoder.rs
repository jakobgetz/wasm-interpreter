use std::io::{Cursor, Read};
use crate::module::Module;

pub struct Decoder;

impl Decoder {
    pub fn decode(byte_code: &[u8]) -> Result<Module, &'static str> {
        let mut cursor = Cursor::new(byte_code);
        // First check version and Magic number
        Self::check_magic_number(&mut cursor)?;
        let version = Self::get_version(&mut cursor)?;
        if version != 1 {
            return Err("Version {version} unsupported. Currently there is only version 1 supported");
        };
        // Decode into Module
        todo!()
        // Return this Module the Typechecker will afterwards typecheck it
    }

    fn check_magic_number(cursor: &mut Cursor<&[u8]>) -> Result<(), &'static str> {
        todo!()
    }

    fn get_version(cursor: &mut Cursor<&[u8]>) -> Result<u32, &'static str> {
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
