//! # wf-evaluator
//!
//! Basically, the evaluator's job is to parse the input arguments,
//! compile the program and run the program. For the input, we deserialize
//! into a list of Values known to the evaluator. We then pass that to
//! the compiled artifact by serializing it to bincode.

use std::env::current_dir;
use std::error::Error;
use std::io::{stdin, BufReader, Write};
use std::process::{Command, Stdio};

use libloading::Library;
use tempfile::NamedTempFile;

mod ty;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub code_string: String,
    pub function_name: String,
    pub function_arguments: serde_json::Map<String, serde_json::Value>,
}

// args to the function template:
// code: the code string
// fn_name: the name of the function
// fn_args: `get_next(&mut values)?`, repeated by number of args

fn main() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(stdin().lock());
    let values = serde_json::Deserializer::from_reader(reader).into_iter::<Input>();
    for input in values {
        let input = input?;
        let code = input.code_string;
        let fn_name = input.function_name;
        let fn_args = "get_next(&mut values)?,".repeat(input.function_arguments.len());
        let compile = format!(
            include_str!("function_template.rs"),
            code = code,
            fn_name = fn_name,
            fn_args = fn_args
        );
        let tmp = NamedTempFile::new()?.into_temp_path();
        let mut rustc = Command::new("rustc")
            .args(["-", "-o"])
            .arg(&tmp)
            .args(["--crate-type", "cdylib"])
            .args(["--edition", "2021"])
            .args(["-L", "dependency=./target/debug/deps"])
            .args(["--extern", "wfrt=./target/debug/libwfrt.rlib"])
            .current_dir(current_dir()?.canonicalize()?)
            .stderr(Stdio::inherit())
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()?;
        rustc.stdin.take().unwrap().write_all(compile.as_bytes())?;
        if !rustc.wait()?.success() {
            todo!("fail compilation")
        }
        
        let lib = unsafe {
            Library::new(&tmp)
        }?;

        let entry = unsafe {
            lib.get::<wfrt::ffi::Function>(b"evaluate_entrypoint\0")
        }?;

        let args = input.function_arguments.into_iter().map(|(_, v)| {
            let ty = ty::Type::from_json(v.get("Z1K1").unwrap()).unwrap();
            let v = ty.to_value(v).unwrap();
            v
        }).collect::<Vec<_>>();

        let result = unsafe {
            entry(wfrt::ffi::Bytes::from_slice(&wfrt::to_stdvec(&args)?))
        };
        let result: Result<wfrt::Value, String> = wfrt::from_bytes(&result.into_vec())?;
        let val = result?;
        dbg!(val);

        // TODO: serialize the value

        lib.close()?;
    }

    Ok(())
}
