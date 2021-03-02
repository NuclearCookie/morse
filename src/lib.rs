/*!
This crate provides a library for encoding and decoding morse code

This crate's documentation provides some simple examples on how to use it.
*/

#![no_std]
#![deny(missing_docs)]

#[macro_use]
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

/// The structure that is returned if the encode or decode functions failed.
#[derive(Debug)]
pub struct TranslationError {
    /// Vec of all unsupported characters causing the error.
    pub unsupported_characters: Vec<String>,
    /// The completed parse result. Failed characters have been replaced by `#`
    pub result: String,
}

pub mod encode;
pub mod decode;
