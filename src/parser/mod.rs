use ::error_creation::res_error;
use ::compiler::hl_abstractions;
//use ::pattern_matching::pmatch_str;
use ::runtime;

use ::std::str::FromStr;
use ::std::collections::HashSet;
use ::std::collections::HashMap;


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

/*
pub enum HllTextToken<'s> {
    Ident(&'s str),
    RawVal(&'s str),
    OpenBrace,
    CloseBrace,
    OpenParenthesis,
    CloseParenthesis,
    Fn,
    Let,
    If,
    Else,
    Arrow,
    Type(&'s str),
    Operator(&'s str),
    Return,
}


pub fn parse_source_str_to_text_tokens<'s, E>(source: &'s str) -> Result<Vec<HllTextToken<'s>>, E>
    where E: FromStr {

    use self::HllTextToken::*;

    let t_enders: HashSet<char> = [' ', '\n'].iter().cloned().collect();

    let mut res_vec = Vec::<HllTextToken<'s>>::new();

    let cursor = 0;
    while cursor < source.len() {
        let token_str = get_token_str(source, cursor, &t_enders);
        let token = match token_str {
            "{" => OpenBrace,
            "}" => CloseBrace,
            "(" => OpenParenthesis,
            ")" => CloseParenthesis,
            "fn" => Fn,
            "let" => Let,
            "if" => If,
            "else" => Else,
            "->" => Arrow,
            _ => {
                panic!("Not implemented!");
            }
        };

        res_vec.push(token);
    }
    
    Ok(res_vec)
}

fn get_token_str<'s>(src: &'s str, cur: usize, t_enders: &HashSet<char>) -> &'s str {

    let mut l = 0;

    let mut it = src.chars();
    it.by_ref().nth(cur);
    let sb = it.as_str();

    for c in src.chars().skip(cur) {
        if t_enders.contains(&c) { break; }
        l += 1;
    }

    let sb_end = sb.char_indices().nth(l).map(|(n, _)| { n }).unwrap_or(0);

    &sb[..sb_end]
}

pub struct HLLStructToken {

}
*/

/*
fn parse_source<'a>(src: &'a mut str) {

    let mut program = hl_abstractions::Program::new();
    let mut current_function: Option<i64> = None;

    match pmatch_str(src, &["fn main"]) {
        Some(S) => {
            program.functions.push(hl_abstractions::Function::new());
            current_function = Some(0);
            src = S[0];
        },
        None => panic!("No main!"),
    }

    while src.len() > 0 {
        match current_function {
            None => { //Global state
                if let Some(S) = pmatch_str(src, &["fn "]) {
                    program.functions.push(hl_abstractions::Function::new());
                    let c_fun = program.functions.len();
                    current_function = Some(c_fun as i64);
                    src = S[c_fun as usize];
                }
            }
            Sone => {
                //TODO: FINISH
            }
        }
    }
}
*/

pub fn parse_llsource<E>(src: &str) -> Result<runtime::program::Program, E>
    where E: FromStr {

    let mut prog = runtime::program::Program::new();

    let mut procedures_map: HashMap<i64, runtime::procedure::Procedure> = HashMap::new();
    let mut v: Vec<&str> = src.rsplit(|c| (c == ' ' || c == '\n' || c == '\t'|| c == '\r')).collect();
    v.retain(|s| *s != "" && *s != "\n" && *s != "\t" && *s != "\r");

    //println!("src is {}", src);
    //println!("tokens are: {:?}", v);

    while v.len() > 0 {

        stack_pop!{ v -> tk };
        match tk {
            "PROC" => {
                let (vr, procedure) = p_ll_match_procedure(v)?;
                procedures_map.insert(procedure.1, procedure.0);
                v = vr;
            },
            _ => return res_error(&("Expected 'PROC', found \'".to_string() + &tk + "\'.")),
        }
    }

    for i in 0..procedures_map.len() {
        match procedures_map.remove(&(i as i64)) {
            Some(v) => prog.procedures.push(v),
            None =>return res_error(&("Procedures must be numbered from 0 to n, where n is the total procedure count - 1. PROC no. ".to_string() + &i.to_string() + "is missing.")),
        }
    }

    Ok(prog)
}

fn p_ll_match_procedure<E>(mut v: Vec<&str>) -> Result<(Vec<&str>, (runtime::procedure::Procedure, i64)), E>
    where E: FromStr {
    
    stack_pop!{ v -> tk1, tk2 };
    let proc_id = match i64::from_str(tk1) {
        Ok(v) => v,
        Err(_) => return res_error(&("Expected integer value, found \'".to_string() + tk1 + "\'")),
    };
    let proc_mem = match i64::from_str(tk2) {
        Ok(v) => v,
        Err(_) => return res_error(&("Expected integer value, found \'".to_string() + tk2 + "\'")),
    };
    let (vr, ptokens) = p_ll_match_proc_tokens(v)?;
    v = vr;

    let procedure = runtime::procedure::Procedure::new(&ptokens, proc_mem);

    Ok((v, (procedure, proc_id)))
}

fn p_ll_match_proc_tokens<E>(mut v: Vec<&str>) -> Result<(Vec<&str>, Vec<runtime::procedure::ProcToken>), E>
    where E: FromStr {

    let mut proc_tokens = Vec::<runtime::procedure::ProcToken>::new();
    
    loop {
        stack_pop!{ v -> tk };
        match tk {
            "PROC_END" => return Ok((v, proc_tokens)),
            "PushVal" => {
                let (vr, pval) = p_ll_match_proc_token_value(v)?;
                proc_tokens.push(runtime::procedure::ProcToken::PushVal(pval));
                v = vr;
            },
            "PushFromMemS" => {
                let (vr, pval) = p_ll_match_proc_token_addr(v)?;
                proc_tokens.push(runtime::procedure::ProcToken::PushFromMemS(pval));
                v = vr;
            },
            "WriteToMemS_NP" => {
                let (vr, pval) = p_ll_match_proc_token_addr(v)?;
                proc_tokens.push(runtime::procedure::ProcToken::WriteToMemS_NP(pval));
                v = vr;
            },
            "PushFromMem" => proc_tokens.push(runtime::procedure::ProcToken::PushFromMem),
            "WriteToMem_NP" => proc_tokens.push(runtime::procedure::ProcToken::WriteToMem_NP),
            "Pop" => proc_tokens.push(runtime::procedure::ProcToken::Pop),
            "MAlloc" => return res_error("Malloc is not supported yet!"),
            "MFree" => return res_error("MFree is not supported yet!"),
            "ExecuteProc" => {
                let (vr, pval) = p_ll_match_proc_token_addr(v)?;
                proc_tokens.push(runtime::procedure::ProcToken::ExecuteProc(pval));
                v = vr;
            },
            "Goto" => {
                let (vr, pval) = p_ll_match_proc_token_addr(v)?;
                proc_tokens.push(runtime::procedure::ProcToken::Goto(pval));
                v = vr;
            },
            "GotoIf" => {
                let (vr, pval) = p_ll_match_proc_token_addr(v)?;
                proc_tokens.push(runtime::procedure::ProcToken::GotoIf(pval));
                v = vr;
            },
            "Return" => proc_tokens.push(runtime::procedure::ProcToken::Return),
            "Inc" => proc_tokens.push(runtime::procedure::ProcToken::Inc),
            "Dec" => proc_tokens.push(runtime::procedure::ProcToken::Dec),
            "Add" => proc_tokens.push(runtime::procedure::ProcToken::Add),
            "Sub" => proc_tokens.push(runtime::procedure::ProcToken::Sub),
            "Mul" => proc_tokens.push(runtime::procedure::ProcToken::Mul),
            "Div" => proc_tokens.push(runtime::procedure::ProcToken::Div),
            "Pow" => proc_tokens.push(runtime::procedure::ProcToken::Pow),
            "Sqrt" => proc_tokens.push(runtime::procedure::ProcToken::Sqrt),
            "Abs" => proc_tokens.push(runtime::procedure::ProcToken::Abs),
            "GR" => proc_tokens.push(runtime::procedure::ProcToken::GR),
            "LSEQ" => proc_tokens.push(runtime::procedure::ProcToken::LSEQ),
            "EQ" => proc_tokens.push(runtime::procedure::ProcToken::EQ),
            "AND" => proc_tokens.push(runtime::procedure::ProcToken::AND),
            "OR" => proc_tokens.push(runtime::procedure::ProcToken::OR),
            "XOR" => proc_tokens.push(runtime::procedure::ProcToken::XOR),
            "NOT" => proc_tokens.push(runtime::procedure::ProcToken::NOT),
            "BitL" => proc_tokens.push(runtime::procedure::ProcToken::BitL),
            "BitR" => proc_tokens.push(runtime::procedure::ProcToken::BitR),
            _ => return res_error(&("Expected instruction, found \'".to_string() + tk + "\'")),
        }
    }
}

fn p_ll_match_proc_token_value<E>(mut v: Vec<&str>) -> Result<(Vec<&str>, runtime::scope::ScopedValue), E>
    where E: FromStr {

    stack_pop!{ v -> tk1, tk2 };
    let t = match tk1 {
        "int" => runtime::scope::ScopedValue::Int(match i32::from_str(tk2) {
            Ok(v) => v,
            Err(_) => return res_error(&("Expected iteger value, found \'".to_string() + tk2 + "\'")),
        }),
        "float" => runtime::scope::ScopedValue::Float(match f64::from_str(tk2) {
            Ok(v) => v,
            Err(_) => return res_error(&("Expected iteger value, found \'".to_string() + tk2 + "\'")),
        }),
        "bool" => runtime::scope::ScopedValue::Bool(match bool::from_str(tk2) {
            Ok(v) => v,
            Err(_) => return res_error(&("Expected iteger value, found \'".to_string() + tk2 + "\'")),
        }),
        _ => return res_error(&("Expected type specifier, found \'".to_string() + tk1 + "\'")),
    };

    Ok((v, t))
}

fn p_ll_match_proc_token_addr<E>(mut v: Vec<&str>) -> Result<(Vec<&str>, i64), E>
    where E: FromStr {
    
    stack_pop!{ v -> tk };
    let val = match i64::from_str(tk) {
        Ok(v) => v,
        Err(_) => return res_error(&("Expected integer value, found \'".to_string() + tk + "\')")),
    };

    Ok((v, val))
}