use crate::{config::Config, decoder::Decoder, module::Module, validator::Validator};
use std::{fs, io::Read};

pub struct Interpreter;

impl Interpreter {
    pub fn run(config: Config) -> Result<(), String> {
        let mut wasm_file = fs::File::open(&config.binary_path).unwrap_or_else(|_| {
            panic!("Error reading file {}", config.binary_path);
        });
        let mut byte_code = Vec::new();
        wasm_file.read_to_end(&mut byte_code).unwrap_or_else(|_| {
            panic!("Error reading bytes from file {}", config.binary_path);
        });
        let module = Decoder::decode(&byte_code).unwrap_or_else(|err| {
            panic!("Error decoding binary: {}", err);
        });
        Validator::validate(&module).unwrap_or_else(|err| {
            panic!("Error validating module: {}", err);
        });
        Self.interpret(module).unwrap_or_else(|err| {
            panic!("Error interpreting binary: {}", err);
        });
        Ok(())
    }

    fn interpret(&self, module: Module) -> Result<(), String> {
        dbg!(module);
        todo!()
    }
}
