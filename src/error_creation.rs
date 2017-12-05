use ::std::str::FromStr;

pub fn error_creation_error<V, T>(_: V) -> T {
    panic!("Couldn't create an error!");
}

pub fn res_error<V, E>(message: &str) -> Result<V, E>
    where E: FromStr {

        Err(E::from_str(message).unwrap_or_else(error_creation_error))
}