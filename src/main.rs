//#[macro_use]
//mod classic_enum;
mod error_creation;
mod pattern_matching;
mod runtime;
mod compiler;
mod parser;


fn main() {

    let bin_digit_cnt_proc_instructions = create_bin_digit_cnt_proc_instructions();
    let bin_digit_cnt_procedure = runtime::procedure::Procedure::new(&bin_digit_cnt_proc_instructions, 4);
    let main_instructions = create_main_instructions();
    let main_procedure = runtime::procedure::Procedure::new(&main_instructions, 0);
    let mut program = runtime::program::Program::new();
    program.procedures.push(main_procedure);
    program.procedures.push(bin_digit_cnt_procedure);
    match program.execute::<String>() {
        Ok(_) => (),
        Err(e) => println!("An error has occured during the execution of BS function: {}", e),
    }

    println!("Stack is: {:?}", program.stack);

    println!("3 ^ 4 = {}", int_pow(3, 4));

    println!("Hello, world!");
}

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

fn create_bin_digit_cnt_proc_instructions() -> Vec<runtime::procedure::ProcToken> {
    use runtime::procedure::ProcToken::*;
    use runtime::scope::ScopedValue::*;
    vec![
        WriteToMemS_NP(0), //v //Write argument v to local memory address 0
        Pop,
        PushVal(Int(31)), //Write 31 t int i in local memory addres 1
        WriteToMemS_NP(1), // i 
        Pop,
        PushFromMemS(0), //v //#005 //Check (v & (1 << i) == 0) & (i >= 0) 
        PushVal(Int(1)),
        PushFromMemS(1), // i;
        BitL,
        AND,
        PushVal(Int(0)),
        EQ,
        PushVal(Int(0)),
        PushFromMemS(1),
        LSEQ,
        AND,
        GotoIf(21), //If the condition is met then go to #021
        PushFromMemS(1), //i //Return i + 1
        PushVal(Int(1)),
        Add,
        Return,
        PushFromMemS(1), //i //#021 //Decrement i and write it to local memor address 1
        PushVal(Int(1)),
        Sub,
        WriteToMemS_NP(1), //i
        Pop,
        Goto(5), //Go to #005
    ]
}

fn create_main_instructions() ->Vec<runtime::procedure::ProcToken> {
    use runtime::procedure::ProcToken::*;
    use runtime::scope::ScopedValue::*;
    vec![
        PushVal(Int(16)),
        ExecuteProc(1),
        Return,
    ]
}

fn get_bin_len(v: i32) -> i32 {
    let mut i: i32 = 31;
    while (v & (1 << i) == 0) & (i >= 0) {
        i = i - 1;
    }
    return i + 1;
}