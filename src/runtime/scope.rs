use error_creation::res_error;

use ::std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum ScopedValue {
    Int(i32),
    Float(f64),
    Bool(bool),
    Empty,
}


use self::ScopedValue::*;

impl ScopedValue {

    pub fn add<E>(v1: Self, v2: Self) -> Result<Self, E>
        where E: FromStr {
       
        match (v1, v2) {
            (Int(i1), Int(i2)) => Ok(Int(i1 + i2)),
            (Float(i1), Float(i2)) => Ok(Float(i1 + i2)),
            _ => res_error("Incompatible values (missing a cast?)")
        }
    }

    pub fn sub<E>(v1: Self, v2: Self) -> Result<Self, E>
        where E: FromStr {
       
        match (v1, v2) {
            (Int(i1), Int(i2)) => Ok(Int(i1 - i2)),
            (Float(i1), Float(i2)) => Ok(Float(i1 - i2)),
            _ => res_error("Incompatible values (missing a cast?)")
        }
    }

    pub fn mul<E>(v1: Self, v2: Self) -> Result<Self, E>
        where E: FromStr {
       
        match (v1, v2) {
            (Int(i1), Int(i2)) => Ok(Int(i1 * i2)),
            (Float(i1), Float(i2)) => Ok(Float(i1 * i2)),
            _ => res_error("Incompatible values (missing a cast?)")
        }
    }

    pub fn div<E>(v1: Self, v2: Self) -> Result<Self, E>
        where E: FromStr {
       
        match (v1, v2){
            (Int(i1), Int(i2)) => Ok(Int(i1 / i2)),
            (Float(i1), Float(i2)) => Ok(Float(i1 / i2)),
            _ => res_error("Incompatible values (missing a cast?)")
        }
    }

    pub fn pow<E>(v1: Self, v2: Self) -> Result<Self, E>
        where E: FromStr {
       
        match (v1, v2) {
            (Int(i1), Int(i2)) => Ok(Int(int_pow(i1, i2))),
            (Float(i1), Float(i2)) => Ok(Float(i1.powf(i2))),
            _ => res_error("Incompatible values (missing a cast?)")
        }
    }

    pub fn sqrt<E>(v1: Self) -> Result<Self, E>
        where E: FromStr {
       
        match v1 {
            Int(i1) => Ok(Int((i1 as f64).sqrt() as i32)),
            Float(i1) => Ok(Float(i1.sqrt())),
            _ => res_error("Incompatible values (missing a cast?)")
        }
    }

    pub fn abs<E>(v1: Self) -> Result<Self, E>
        where E: FromStr {
       
        match v1 {
            Int(i1) => Ok(Int(i1.abs())),
            Float(i1) => Ok(Float(i1.abs())),
            _ => res_error("Incompatible values (missing a cast?)")
        }
    }

    pub fn gr<E>(v1: Self, v2: Self) -> Result<Self, E>
        where E: FromStr {
       
        match (v1, v2) {
            (Int(i1), Int(i2)) => Ok(Bool(i1 > i2)),
            (Float(i1), Float(i2)) => Ok(Bool(i1 > i2)),
            _ => res_error("Incompatible values (missing a cast?)")
        }
    }

    pub fn lseq<E>(v1: Self, v2: Self) -> Result<Self, E>
        where E: FromStr {
       
        match (v1, v2) {
            (Int(i1), Int(i2)) => Ok(Bool(i1 <= i2)),
            (Float(i1), Float(i2)) => Ok(Bool(i1 <= i2)),
            _ => res_error("Incompatible values (missing a cast?)")
        }
    }

    pub fn eq<E>(v1: Self, v2: Self) -> Result<Self, E>
        where E: FromStr {
       
        match (v1, v2) {
            (Int(i1), Int(i2)) => Ok(Bool(i1 == i2)),
            (Float(i1), Float(i2)) => Ok(Bool(i1 == i2)),
            (Bool(i1), Bool(i2)) => Ok(Bool(i1 == i2)),
            _ => res_error("Incompatible values (missing a cast?)")
        }
    }

    pub fn and<E>(v1: Self, v2: Self) -> Result<Self, E>
        where E: FromStr {

        match (v1, v2) {
            (Int(i1), Int(i2)) => Ok(Int(i1 & i2)),
            (Bool(i1), Bool(i2)) => Ok(Bool(i1 & i2)),
            _ => res_error("Incompatible values (missing a cast?)")
        }
    }

    pub fn or<E>(v1: Self, v2: Self) -> Result<Self, E>
        where E: FromStr {

        match (v1, v2) {
            (Int(i1), Int(i2)) => Ok(Int(i1 | i2)),
            (Bool(i1), Bool(i2)) => Ok(Bool(i1 | i2)),
            _ => res_error("Incompatible values (missing a cast?)")
        }
    }

    pub fn xor<E>(v1: Self, v2: Self) -> Result<Self, E>
        where E: FromStr {

        match (v1, v2) {
            (Int(i1), Int(i2)) => Ok(Int((i1 | i2) & !(i1 & i2))),
            (Bool(i1), Bool(i2)) => Ok(Bool((i1 | i2) & !(i1 & i2))),
            _ => res_error("Incompatible values (missing a cast?)")
        }
    }

    pub fn not<E>(v1: Self) -> Result<Self, E>
        where E: FromStr {

        match v1 {
            Int(i1) => Ok(Int(!i1)),
            Bool(i1) => Ok(Bool(!i1)),
            _ => res_error("Incompatible values (missing a cast?)")
        }
    }

    pub fn bitl<E>(v1: Self, v2: Self) -> Result<Self, E>
        where E: FromStr {

            match (v1, v2) {
                (Int(i1), Int(i2)) => Ok(Int(i1 << i2)),
                _ => res_error("Incompatible values (missing a cast?)")
            }
    }

    pub fn bitr<E>(v1: Self, v2: Self) -> Result<Self, E>
        where E: FromStr {

            match (v1, v2) {
                (Int(i1), Int(i2)) => Ok(Int(i1 >> i2)),
                _ => res_error("Incompatible values (missing a cast?)")
            }
    }

    pub fn get_bool<E>(&self) -> Result<bool, E>
        where E: FromStr {

        match *self {
            Bool(v) => Ok(v),
            _ => res_error("Not a bool")
        }
    }

    pub fn get_int<E>(&self) -> Result<i32, E>
        where E: FromStr {

        match *self {
            Int(v) => Ok(v),
            _ => res_error("Not a bool")
        }
    }

    pub fn get_float<E>(&self) -> Result<f64, E>
        where E: FromStr {

        match *self {
            Float(v) => Ok(v),
            _ => res_error("Not a bool")
        }
    }
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

/*
impl ::std::fmt::Display for ScopedValue {

    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {

        use self::ScopedValue::*;

        match *self {
            Int(v) => write!(f, "Int({})", v),
            Float(v) => write!(f, "Float({})", v),
            Bool(v) => write!(f, "Bool({})", v),
            Empty => write!(f, "Empty"),
        }
    }
}
*/