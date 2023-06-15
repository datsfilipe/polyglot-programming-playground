fn error_me(throw: bool) -> Result<(), String> {
    if throw {
        Err("I'm an error".to_string())
    } else {
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let value = error_me(false)?;
    // the above "?" do the same as the below
    let _value_2 = match error_me(false) {
        Err(e) => return Err(e),
        Ok(v) => v,
    };

    return Ok(value);
}
