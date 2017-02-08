#[derive(Debug)]
pub struct TranslationError {
    pub unsupported_characters: Vec<String>,
    pub result: String
}

pub mod encode;
pub mod decode;