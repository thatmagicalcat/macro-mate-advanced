use std::fs;
use std::process::exit;

use functions::Controller;
use mlua::prelude::*;

mod functions;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        println!("Usage: {} <macro file>", args.first().unwrap());
        exit(0);
    }

    let script = std::path::Path::new(&args[1]);

    if !script.exists() {
        println!("[Error]: Script doesn't exists");
        std::process::exit(0);
    }

    let script_contents = fs::read_to_string(script).unwrap_or_else(|e| {
        println!("[Error]: failed to read file, error message: \"{e:?}\"");
        exit(1);
    });

    let lua = Lua::new();
    let function_table = lua.create_table().unwrap();

    let controller = Controller::new(&lua, &function_table);
    controller.register_functions();

    lua.globals().set("mm", function_table).unwrap();
    lua.load(script_contents).exec().unwrap();
}
