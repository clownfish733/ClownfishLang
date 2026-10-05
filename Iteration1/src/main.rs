#![allow(unused)]

//TODO: for now this takes both the config and the programm and outputs the programms executable
//later i want to generate the configured compiler to the user

mod user;
use user::UserIO;

mod errors;
use errors::CompilerError;

//src -> prep src
mod preprocess;
use preprocess::preprocess;

//prep src -> tokens
mod tokenize;
use tokenize::lex;

//tokens -> ast
mod ast;
use ast::parse_tokens;

//ast -> ir
mod ir;
use ir::parse_ast;

//ir -> asm
mod asm;
use asm::{parse_asm, parse_ir};

fn main() {
    let mut user = match UserIO::parse_args() {
        Some(u) => u,
        None => return,
    };

    let src = user.get_src();
    let p_src = match preprocess(&src) {
        Ok(p) => p,
        Err(e) => {
            e.message(&src);
            return;
        }
    };
    let tokens = lex(&p_src);
    let ast = match parse_tokens(tokens) {
        Ok(a) => a,
        Err(e) => {
            e.message(&src);
            return;
        }
    };
    let ir = match parse_ast(ast) {
        Ok(i) => i,
        Err(e) => {
            e.message(&src);
            return;
        }
    };
    let asm = match parse_ir(ir) {
        Ok(a) => a,
        Err(e) => {
            e.message(&src);
            return;
        }
    };
    let code = match parse_asm(asm) {
        Ok(c) => c,
        Err(e) => {
            e.message(&src);
            return;
        }
    };

    user.add_code(code);
    user.output();
}
