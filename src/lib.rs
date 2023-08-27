use std::{fs, io::{Read, Cursor}};

pub struct Interpreter;

impl Interpreter {
    pub fn run(config: Config) -> Result<(), &'static str> {
        let mut wasm_file = fs::File::open(config.binary_path).unwrap();
        let mut byte_code = Vec::new();
        if let Err(_) = wasm_file.read_to_end(&mut byte_code) {
            return Err("Error opening file");
        }
        Parser::parse(&byte_code)?;
        Ok(())
    }
}

pub struct Parser;

impl Parser {
    pub fn parse(byte_code: &[u8]) -> Result<Module, &'static str> {
        let cursor = Cursor::new(byte_code);
        let mut int_buffer = [0; 4];
        cursor.take(4).read_exact(&mut int_buffer);
        let int_value = i32::from_le_bytes(int_buffer);
    }
}

pub struct Config {
    binary_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() > 2 {
            return Err("Too many params");
        }
        if args.len() < 2 {
            return Err("Not enough params");
        }
        let binary_path = args[1].clone();
        Ok(Self { binary_path })
    }
}

pub struct Module {
    version: u8,
    functions: Vec<Function>,
    memory: Memory,
    table: Table,
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

pub struct Memory;

pub struct Table;
