use std::{env, process};
use wasm_interpreter::{Config, Interpreter};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    Interpreter::run(config).unwrap_or_else(|err| {
        println!("Problem interpreting binary {err}");
        process::exit(1);
    });
}
