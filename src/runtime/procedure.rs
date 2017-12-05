/*
Władcy P O T Ę Ż N E J Polski:

Popiel
Piast
Siemowit
Lestek
Siemomysł
Mieszko I
Bolesłąw Chrobry
Mieszko II
Kazimierz Odnowiciel
Bolesław Krzywousty
ROZBICIE
Władysław Łokietek
Kazimierz Wielki
Ludwik Andegaweński
Św Jadwiga
Władysław Jagiełło
Władysław Warneńczyk
Kazimierz JAgiellończyk
Jan Olbracht
Aleksander Jagiellończyk
Zygmunt Stary
Zygmunt August
Henryk Walezy
Stefan Batory
Zygmunt III Waza
Władysław IV
Jan Kazimiesz
Michał Korybut Wiśniowiecki
Jan III Sobieski
August II Mocny
Stanisław Leszczeński
August II Sas
Stanisław August Poniatowski
ZABORY
*/

use super::scope as scope_;
use error_creation::res_error;

use ::std::str::FromStr;


macro_rules! stack_pop {
    {$stack : ident -> $($var: ident),+ } => {
        $(
            let $var = match $stack.pop() {
                Some(v) => v,
                None => return res_error("Stack is empty"),
            };
        )+
    }
}

//Pretty much an assembler
//S stands for static: these have some static parameters
//NP stands for non-pop: these do not pop anything from the stack
#[derive(Copy, Clone, Debug)]
pub enum ProcToken {
    //Basic memory operations
    PushVal(scope_::ScopedValue),
    PushFromMemS(i64),
    WriteToMemS_NP(i64),
    PushFromMem,
    WriteToMem_NP,
    Pop,
    MAlloc,
    MFree,
    //Basic flow control operations
    ExecuteProc(i64),
    Goto(i64),
    GotoIf(i64),
    Return,
    //Basic arithmethics operations
    Inc,
    Dec,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Sqrt,
    Abs,
    GR,
    LSEQ,
    EQ,
    AND,
    OR,
    XOR,
    NOT,
    BitL,
    BitR,
}

pub struct Procedure {
    pub proc_tokens: Vec<ProcToken>,
    pub scope_size: i64,
}

impl Procedure {
    
    pub fn execute<E>(&self, mut stack: Vec<scope_::ScopedValue>, proc_list: &Vec<Procedure>) -> Result<Vec<scope_::ScopedValue>, E>
    where E: FromStr {

        let mut scope: Vec<scope_::ScopedValue> = vec![scope_::ScopedValue::Empty; self.scope_size as usize];

        use self::ProcToken::*;

        let mut ct_id: i64 = 0;
        while ct_id < self.proc_tokens.len() as i64 {

            let p_token = &self.proc_tokens[ct_id as usize];
            println!("Currently processing token {}: {:?}", ct_id, p_token);
            ct_id += 1;

            match *p_token {
                PushVal(ref v) => stack.push(*v),
                PushFromMemS(addr) => stack.push(scope[addr as usize]),
                WriteToMemS_NP(addr) => {
                    match stack.last() {
                        Some(v) => scope[addr as usize] = *v,
                        None => return res_error("Stack is empty"),
                    }
                },
                PushFromMem => {
                    stack_pop!{ stack -> v1 };
                    let addr = v1.get_int()? as usize;
                    if addr >= scope.len() {
                        return res_error("Index out of bounds!");
                    }
                    stack.push(scope[addr]);
                }
                WriteToMem_NP => {
                    let len = stack.len();
                    if len < 2 {
                        return res_error("Too few element on the stack");
                    }
                    let value = stack[len - 2];
                    let addr = stack[len - 1].get_int()? as usize;
                    if addr >= scope.len() {
                        return res_error("Index out of bounds!");
                    }
                    scope[addr] = value;
                }
                Pop => { stack.pop(); },
                ExecuteProc(proc_id) => {
                    println!("Entering procedure {}", proc_id);
                    stack = proc_list[proc_id as usize].execute(stack, &proc_list)?;
                    println!("Procedure {} end.", proc_id);

                },
                Goto(p) => ct_id = p,
                GotoIf(p) => {
                    stack_pop!{ stack -> v1 };
                    if v1.get_bool()? {
                        ct_id = p;
                    }
                }
                Return => return Ok(stack),
                Add => {
                    stack_pop!{ stack -> v1, v2 };
                    let res = scope_::ScopedValue::add(v2, v1)?;
                    stack.push(res);
                }
                Sub => {
                    stack_pop!{ stack -> v1, v2 };
                    let res = scope_::ScopedValue::sub(v2, v1)?;
                    stack.push(res);
                }
                Mul => {
                    stack_pop!{ stack -> v1, v2 };
                    let res = scope_::ScopedValue::mul(v2, v1)?;
                    stack.push(res);
                }
                Div => {
                    stack_pop!{ stack -> v1, v2 };
                    let res = scope_::ScopedValue::div(v2, v1)?;
                    stack.push(res);
                }
                Pow => {
                    stack_pop!{ stack -> v1, v2 };
                    let res = scope_::ScopedValue::pow(v2, v1)?;
                    stack.push(res);
                }
                Sqrt => {
                    stack_pop!{ stack -> v1 };
                    let res = scope_::ScopedValue::sqrt(v1)?;
                    stack.push(res);
                }
                Abs => {
                    stack_pop!{ stack -> v1 };
                    let res = scope_::ScopedValue::abs(v1)?;
                    stack.push(res);
                }
                GR => {
                    stack_pop!{ stack -> v1, v2 };
                    let res = scope_::ScopedValue::gr(v2, v1)?;
                    stack.push(res);
                }
                LSEQ => {
                    stack_pop!{ stack -> v1, v2 };
                    let res = scope_::ScopedValue::lseq(v2, v1)?;
                    stack.push(res);
                }
                EQ => {
                    stack_pop!{ stack -> v1, v2 };
                    let res = scope_::ScopedValue::eq(v2, v1)?;
                    stack.push(res);
                }
                AND => {
                    stack_pop!{ stack -> v1, v2};
                    let res = scope_::ScopedValue::and(v2, v1)?;
                    stack.push(res);
                }
                OR => {
                    stack_pop!{ stack -> v1, v2};
                    let res = scope_::ScopedValue::or(v2, v1)?;
                    stack.push(res);
                }
                XOR => {
                    stack_pop!{ stack -> v1, v2};
                    let res = scope_::ScopedValue::xor(v2, v1)?;
                    stack.push(res);
                }
                NOT => {
                    stack_pop!{ stack -> v1 };
                    let res = scope_::ScopedValue::not(v1)?;
                    stack.push(res);
                }
                BitL => {
                    stack_pop!{ stack -> v1, v2 };
                    let res = scope_::ScopedValue::bitl(v2, v1)?;
                    stack.push(res);
                }
                BitR => {
                    stack_pop!{ stack -> v1, v2 };
                    let res = scope_::ScopedValue::bitr(v2, v1)?;
                    stack.push(res);
                }
                _ => panic!("Not implemented!"),
            }
        }

        Ok(stack)
    }

    pub fn new(proc_tokens: &Vec<ProcToken>, scope_size: i64) -> Self {
        
        Procedure {
            proc_tokens: proc_tokens.clone(),
            scope_size,
        }
    }
}