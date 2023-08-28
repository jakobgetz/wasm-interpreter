// SPEC
pub const MAGIC: [u8; 4] = [0x00, 0x61, 0x73, 0x6d];

// Section Codes
pub mod section {
    pub const CUSTOM: u8 = 0;
    pub const TYPE: u8 = 1;
    pub const IMPORT: u8 = 2;
    pub const FUNCTION: u8 = 3;
    pub const TABLE: u8 = 4;
    pub const MEMORY: u8 = 5;
    pub const GLOBAL: u8 = 6;
    pub const EXPORT: u8 = 7;
    pub const START: u8 = 8;
    pub const ELEMENT: u8 = 9;
    pub const CODE: u8 = 10;
    pub const DATA: u8 = 11;
}

pub mod types {
    pub const I32: u8 = 0x7F;
    pub const I64: u8 = 0x7E;
    pub const F32: u8 = 0x7D;
    pub const F64: u8 = 0x7C;
    pub const FUNCREF: u8 = 0x70;
    pub const FUNCTION: u8 = 0x60;
    pub const RESULT: u8 = 0x40;
    pub const CONST: u8 = 0x00;
    pub const VAR: u8 = 0x01;
}


pub mod import_desc {
    pub const TYPE: u8 = 0x00;
    pub const TABLE: u8 = 0x01;
    pub const MEM: u8 = 0x02;
    pub const GLOBAL: u8 = 0x03;
}

pub mod expr {
    pub const END: u8 = 0x0B;
}

pub mod instr {
    pub const UNREACHABLE: u8 = 0x00;
    pub const NOP : u8 = 0x01;
    pub const BLOCK: u8 = 0x02;
    pub const LOOP: u8 = 0x03;
    pub const IF: u8 = 0x04;
    pub const ELSE: u8 = 0x05;
    pub const BR: u8 = 0x0C;
    pub const BR_IF: u8 = 0x0D;
    pub const BR_TABLE: u8 = 0x0E;
    pub const RETURN: u8 = 0x0F;
    pub const CALL: u8 = 0x10;
    pub const CALL_INDIRECT: u8 = 0x11;
    pub const DROP: u8 = 0x1A;
    pub const SELECT: u8 = 0x1B;
    pub const LOCAL_GET: u8 = 0x20;
    pub const LOCAL_SET: u8 = 0x21;
    pub const LOCAL_TEE: u8 = 0x22;
    pub const GLOBAL_GET: u8 = 0x23;
    pub const GLOBAL_SET: u8 = 0x24;
    pub const I32_LOAD: u8 = 0x28;
    pub const I64_LOAD: u8 = 0x29;
    pub const F32_LOAD: u8 = 0x2A;
    pub const F64_LOAD: u8 = 0x2B;
    pub const I32_LOAD8_S: u8 = 0x2C;
    pub const I32_LOAD8_U: u8 = 0x2D;
    pub const I32_LOAD16_S: u8 = 0x2E;
    pub const I32_LOAD16_U: u8 = 0x2F;
    pub const I64_LOAD8_S: u8 = 0x30;
    pub const I64_LOAD8_U: u8 = 0x31;
    pub const I64_LOAD16_S: u8 = 0x32;
    pub const I64_LOAD_16_U: u8 = 0x33;
    // pub const : u8 = 0x;
    

    
    pub const I32_ADD: u8 = 0x6A;
}

