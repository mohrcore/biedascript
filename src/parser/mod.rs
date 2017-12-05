use ::error_creation::res_error;
use ::compiler::hl_abstractions;
use ::pattern_matching::pmatch_str;

use ::std::str::FromStr;
use ::std::collections::HashSet;


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