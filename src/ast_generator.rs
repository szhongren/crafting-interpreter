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
    file_buffer.write_fmt(format_args!("abstract class {} {{\n", base_name))?;

    for lox_type in types {
        let mut type_split = lox_type.split(":");
        let class_name = type_split.next().unwrap().trim();
        let field_list = type_split.next().unwrap().trim();
        define_type(&mut file_buffer, base_name, class_name, field_list)?;
    }
    file_buffer.write(b"}\n")?;
    Ok(())
}

fn define_type(
    file_buffer: &mut File,
    base_name: &str,
    class_name: &str,
    field_list: &str,
) -> Result<()> {
    file_buffer.write_fmt(format_args!(
        "  static class {} extends {} {{\n",
        class_name, base_name
    ))?;

    // constructor
    file_buffer.write_fmt(format_args!("    {}({}) {{\n", class_name, field_list))?;

    // store params in fields
    let fields = field_list.split(", ");
    for field in fields.clone() {
        let name = field.split(" ").next().unwrap();
        file_buffer.write_fmt(format_args!("      this.{} = {};\n", name, name))?;
    }
    file_buffer.write(b"    }\n")?;

    // fields
    for field in fields {
        file_buffer.write_fmt(format_args!("    final {};\n", field))?;
    }

    file_buffer.write(b"  }\n")?;
    Ok(())
}
