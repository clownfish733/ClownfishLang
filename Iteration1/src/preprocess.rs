use crate::errors::CompilerError;

pub fn preprocess(src: &str) -> Result<String, CompilerError> {
    Ok(src.to_string())
}
