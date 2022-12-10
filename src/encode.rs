//! Ascii to Morse
use crate::TranslationError;

use alloc::string::{String,ToString};

/// Encodes an ascii string into a morse code representation
///
/// # Examples
/// ```
/// use morse::encode;
///
/// assert_eq!(encode::encode("sos").unwrap(), "... ___ ...");
/// ```
/// # Errors
///
/// Encoding will error with a `morse::TranslationError`
/// when an unsupported character is being encoded.
/// The error structure contains a `Vec<String> unsupported_characters`
/// to show what characters failed.
/// Note: The input will still complete parsing.
pub fn encode<S: Into<String>>(input: S) -> Result<String, TranslationError> {
    let text = input.into().to_lowercase().trim().to_string();
    let chars = text.chars();
    let mut result = String::new();
    let mut error_values = vec![];
    for c in chars {
        let code = match c {
            'a' => "._",
            'b' => "_...",
            'c' => "_._.",
            'd' => "_..",
            'e' => ".",
            'f' => ".._.",
            'g' => "__.",
            'h' => "....",
            'i' => "..",
            'j' => ".___",
            'k' => "_._",
            'l' => "._..",
            'm' => "__",
            'n' => "_.",
            'o' => "___",
            'p' => ".__.",
            'q' => "__._",
            'r' => "._.",
            's' => "...",
            't' => "_",
            'u' => ".._",
            'v' => "..._",
            'w' => ".__",
            'x' => "_.._",
            'y' => "_.__",
            'z' => "__..",
            #[cfg(feature = "norwegian")]
            'æ' => "._._",
            #[cfg(feature = "norwegian")]
            'ø' => "___.",
            #[cfg(feature = "norwegian")]
            'å' => ".__._",
            '0' => "_____",
            '1' => ".____",
            '2' => "..___",
            '3' => "...__",
            '4' => "...._",
            '5' => ".....",
            '6' => "_....",
            '7' => "__...",
            '8' => "___..",
            '9' => "____.",
            '.' => "._._._",
            ',' => "__..__",
            '?' => "..__..",
            '\'' => ".____.",
            '!' => "_._.__",
            '/' => "_.._.",
            '(' => "_.__.",
            ')' => "_.__._",
            '&' => "._...",
            ':' => "___...",
            ';' => "_._._.",
            '=' => "_..._",
            '+' => "._._.",
            '-' => "_...._",
            '_' => "..__._",
            '"' => "._.._.",
            '$' => "..._.._",
            '@' => ".__._.",
            ' ' => "/",
            _ => {
                error_values.push(c.to_string());
                "#"
            }
        };
        result.push_str(code);
        result.push(' ');
    }
    // remove final separator.
    result.pop();
    if error_values.len() == 0 {
        Ok(result)
    } else {
        Err(TranslationError {
            unsupported_characters: error_values,
            result: result,
        })
    }
}

#[test]
fn encode_lower_case_letters() {
    assert_eq!("._", encode("a").unwrap());
    assert_eq!("_...", encode("b").unwrap());
    assert_eq!("_._.", encode("c").unwrap());
    assert_eq!("_..", encode("d").unwrap());
    assert_eq!(".", encode("e").unwrap());
    assert_eq!(".._.", encode("f").unwrap());
    assert_eq!("__.", encode("g").unwrap());
    assert_eq!("....", encode("h").unwrap());
    assert_eq!("..", encode("i").unwrap());
    assert_eq!(".___", encode("j").unwrap());
    assert_eq!("_._", encode("k").unwrap());
    assert_eq!("._..", encode("l").unwrap());
    assert_eq!("__", encode("m").unwrap());
    assert_eq!("_.", encode("n").unwrap());
    assert_eq!("___", encode("o").unwrap());
    assert_eq!(".__.", encode("p").unwrap());
    assert_eq!("__._", encode("q").unwrap());
    assert_eq!("._.", encode("r").unwrap());
    assert_eq!("...", encode("s").unwrap());
    assert_eq!("_", encode("t").unwrap());
    assert_eq!(".._", encode("u").unwrap());
    assert_eq!("..._", encode("v").unwrap());
    assert_eq!(".__", encode("w").unwrap());
    assert_eq!("_.._", encode("x").unwrap());
    assert_eq!("_.__", encode("y").unwrap());
    assert_eq!("__..", encode("z").unwrap());
    #[cfg(feature = "norwegian")]
    {
        assert_eq!("._._", encode("æ").unwrap());
        assert_eq!("___.", encode("ø").unwrap());
        assert_eq!(".__._", encode("å").unwrap());
    }
}

#[test]
fn encode_upper_case_letters() {
    assert_eq!("._", encode("A").unwrap());
    assert_eq!("_...", encode("B").unwrap());
    assert_eq!("_._.", encode("C").unwrap());
    assert_eq!("_..", encode("D").unwrap());
    assert_eq!(".", encode("E").unwrap());
    assert_eq!(".._.", encode("F").unwrap());
    assert_eq!("__.", encode("G").unwrap());
    assert_eq!("....", encode("H").unwrap());
    assert_eq!("..", encode("I").unwrap());
    assert_eq!(".___", encode("J").unwrap());
    assert_eq!("_._", encode("K").unwrap());
    assert_eq!("._..", encode("L").unwrap());
    assert_eq!("__", encode("M").unwrap());
    assert_eq!("_.", encode("N").unwrap());
    assert_eq!("___", encode("O").unwrap());
    assert_eq!(".__.", encode("P").unwrap());
    assert_eq!("__._", encode("Q").unwrap());
    assert_eq!("._.", encode("R").unwrap());
    assert_eq!("...", encode("S").unwrap());
    assert_eq!("_", encode("T").unwrap());
    assert_eq!(".._", encode("U").unwrap());
    assert_eq!("..._", encode("V").unwrap());
    assert_eq!(".__", encode("W").unwrap());
    assert_eq!("_.._", encode("X").unwrap());
    assert_eq!("_.__", encode("Y").unwrap());
    assert_eq!("__..", encode("Z").unwrap());
    #[cfg(feature = "norwegian")]
    {
        assert_eq!("._._", encode("Æ").unwrap());
        assert_eq!("___.", encode("Ø").unwrap());
        assert_eq!(".__._", encode("Å").unwrap());
    }
}

#[test]
fn encode_numbers() {
    assert_eq!(".____", encode("1").unwrap());
    assert_eq!("..___", encode("2").unwrap());
    assert_eq!("...__", encode("3").unwrap());
    assert_eq!("...._", encode("4").unwrap());
    assert_eq!(".....", encode("5").unwrap());
    assert_eq!("_....", encode("6").unwrap());
    assert_eq!("__...", encode("7").unwrap());
    assert_eq!("___..", encode("8").unwrap());
    assert_eq!("____.", encode("9").unwrap());
    assert_eq!("_____", encode("0").unwrap());
}

#[test]
fn encode_other() {
    assert_eq!("._._._", encode(".").unwrap());
    assert_eq!("__..__", encode(",").unwrap());
    assert_eq!("..__..", encode("?").unwrap());
    assert_eq!(".____.", encode("'").unwrap());
    assert_eq!("_._.__", encode("!").unwrap());
    assert_eq!("_.._.", encode("/").unwrap());
    assert_eq!("_.__.", encode("(").unwrap());
    assert_eq!("_.__._", encode(")").unwrap());
    assert_eq!("._...", encode("&").unwrap());
    assert_eq!("___...", encode(":").unwrap());
    assert_eq!("_._._.", encode(";").unwrap());
    assert_eq!("_..._", encode("=").unwrap());
    assert_eq!("._._.", encode("+").unwrap());
    assert_eq!("_...._", encode("-").unwrap());
    assert_eq!("..__._", encode("_").unwrap());
    assert_eq!("._.._.", encode("\"").unwrap());
    assert_eq!("..._.._", encode("$").unwrap());
    assert_eq!(".__._.", encode("@").unwrap());
}

#[test]
fn encode_word() {
    assert_eq!("._ _... _._.", encode("abc").unwrap());
}

#[test]
fn encode_multiple_words() {
    assert_eq!("._ _... _._. / _.. . .._.", encode("abc def").unwrap());
}

#[test]
fn result_err() {
    let morse = match encode("~Hello!") {
        Ok(x) => {
            assert!(false);
            x
        }
        Err(e) => {
            assert_eq!(e.unsupported_characters.len(), 1);
            assert_eq!(e.unsupported_characters[0], "~");
            assert_eq!("# .... . ._.. ._.. ___ _._.__", e.result);
            e.result
        }
    };
    assert_eq!("# .... . ._.. ._.. ___ _._.__", morse);
}

#[test]
fn result_ok() {
    let morse = match encode("Hello!") {
        Ok(x) => {
            assert_eq!(".... . ._.. ._.. ___ _._.__", x);
            x
        }
        Err(e) => {
            assert!(false);
            e.result
        }
    };
    assert_eq!(".... . ._.. ._.. ___ _._.__", morse);
}
