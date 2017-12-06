//#[macro_use]
//mod classic_enum;
mod error_creation;
mod pattern_matching;
mod runtime;
mod compiler;
mod parser;

use std::fs::File;
use std::io::prelude::*;


fn read_line(stdin: &mut std::io::Stdin) -> String {
    stdin.lock().lines().next().unwrap().unwrap()
}

fn main() {

    let mut stdin = std::io::stdin();

    println!("Source path: ");

    let path = read_line(&mut stdin);

    let mut file = File::open(path)
        .expect("File not found!");
    
    let mut source = String::new();
    file.read_to_string(&mut source)
        .expect("Failed to read the source!");

    let mut program = parser::parse_llsource::<String>(&source)
        .unwrap_or_else(|e| panic!("Failed to parse program, the error is: {}", e));

    program.execute::<String>(false)
        .unwrap_or_else(|e| panic!("An error has occured during the execution of the program: {}", e));

    println!("Stack is: {:?}", program.stack);
}

/*
fn int_pow(b: i32, e: i32) -> i32 {
    let mut w = 1;
    let l = 32 - e.leading_zeros();
    for i in 0..(l + 1) {
        let c = e & (1 << (l - i));
        if c == 0 {
            w *= w;
        }
        else {
            w *= w * b;
        }
    }
    w
}
*/

fn get_bin_len(v: i32) -> i32 {
    let mut i: i32 = 31;
    while (v & (1 << i) == 0) & (i >= 0) {
        i = i - 1;
    }
    return i + 1;
}