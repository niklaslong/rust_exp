#[macro_use] extern crate rustler;
// #[macro_use] extern crate rustler_codegen;
#[macro_use] extern crate lazy_static;

use rustler::{NifEnv, NifTerm, NifResult, NifEncoder};

mod atoms {
    rustler_atoms! {
        atom ok;
        //atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler_export_nifs! {
    "Elixir.RustExp.Math",
    [
        ("add", 2, add),
        ("add_tuple", 1, add_tuple),
        ("subtract", 2, subtract),
        ("multiply", 2, multiply),
        ("divide", 2, divide),
        ("repeat", 1, repeat),
        ("put_string", 1, put_string)
    
    ],
    None
}

fn add<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let num1: i64 = try!(args[0].decode());
    let num2: i64 = try!(args[1].decode());

    Ok((atoms::ok(), num1 + num2).encode(env))
}

fn add_tuple<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let tup: (i64, i64) = try!(args[0].decode());
    let (a, b) = tup;

    Ok((atoms::ok(), a + b).encode(env))
}

fn multiply<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let num1: i64 = try!(args[0].decode());
    let num2: i64 = try!(args[1].decode());

    Ok((atoms::ok(), num1 * num2).encode(env))
}

fn divide<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let num1: i64 = try!(args[0].decode());
    let num2: i64 = try!(args[1].decode());

    Ok((atoms::ok(), num1 / num2).encode(env))
}

fn subtract<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let num1: i64 = try!(args[0].decode());
    let num2: i64 = try!(args[1].decode());

    Ok((atoms::ok(), num1 - num2).encode(env))
}

fn repeat<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let str1: String = try!(args[0].decode());

    Ok((atoms::ok(), str1).encode(env))
}

fn put_string<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let str1: String = try!(args[0].decode());
    let result: String = format!("You said: {}", str1);

    Ok((atoms::ok(), result).encode(env))
}
