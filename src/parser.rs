use std::io::{Cursor, Read, Seek, SeekFrom};

use crate::module::Module;

pub struct Parser;

impl Parser {
    pub fn parse(byte_code: &[u8]) -> Result<Module, &'static str> {
        let mut cursor = Cursor::new(byte_code);
        let mut byte_buffer = [0; 1];
        let mut int_buffer = [0; 4];
        if let Err(_) = cursor.read_exact(&mut int_buffer) {
            return Err("Could not read the first 4 bytes of the binary");
        }
        if i32::from_le_bytes(int_buffer) != i32::from_le_bytes([0x00, 0x61, 0x73, 0x6d]) {
            return Err("Wrong magic number");
        }
        if let Err(_) = cursor.read_exact(&mut int_buffer) {
            return Err("Could not read version of the binary");
        }
        let version = i32::from_le_bytes(int_buffer);
        if let Err(_) = cursor.read_exact(&mut byte_buffer) {
            return Ok(Module {
                version,
                functions: Vec::new(),
            });
        }
        let current_byte = byte_buffer[0];
        if current_byte == 0x00 {
            Parser::parse_section_name(&mut cursor);
        }

        todo!();
    }

    fn parse_section_name(cursor: &mut Cursor<&[u8]>) -> Result<(), &'static str> {
        Err("Section name not supported!")
    }
}
