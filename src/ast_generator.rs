use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::io::Result;
use std::path::Path;

pub fn define_ast(output_dir: &str, base_name: &str, types: Vec<&str>) -> Result<()> {
    let directory = Path::new(".").join(output_dir);
    create_dir_all(directory.clone())?;
    let file_path = directory.join(format!("{}{}", base_name, ".rs"));
    let mut file_buffer = File::create(file_path)?;
    file_buffer.write(b"package com.craftinginterpreters.lox;\n\n")?;
    file_buffer.write(b"import java.util.List;\n\n")?;
    file_buffer.write_fmt(format_args!("abstract class {} {{", base_name))?;

    for lox_type in types {
        let mut type_split = lox_type.split(":");
        let class_name = type_split.next().unwrap().trim();
        let fields = type_split.next().unwrap().trim();
        define_type(&file_buffer, base_name, class_name, fields);
    }
    file_buffer.write(b"}\n")?;
    Ok(())
}

fn define_type(file_buffer: &File, base_name: &str, class_name: &str, fields: &str) {
    println!("{}, {}, {}", base_name, class_name, fields);
}
