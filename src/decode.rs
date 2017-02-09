use TranslationError;

/// Decodes a morse representation string into an ascii string
///
/// # Examples
/// ```
/// use morse::decode;
///
/// assert_eq!(decode::decode("... ___ ...").unwrap(), "sos");
/// ```
/// # Errors
///
/// Decoding will error when an unsupported morse character is being decoded.
/// The error structure contains a `Vec<String> unsupported_characters` to show what characters failed.
pub fn decode<S : Into<String>>(input:S) -> Result<String, TranslationError> {
    let text = input.into().replace("*", ".").replace("-","_").trim().to_string();
    let mut result = String::new();
    let mut error_values = vec![];
    let words = text.split("/");
    for word in words {
        let chars = word.trim().split(" ");
        for c in chars {
            let letter = match c {
                "._" => 'a',
                "_..." => 'b',
                "_._." => 'c',
                "_.." => 'd',
                "." => 'e',
                ".._." => 'f',
                "__." => 'g',
                "...." => 'h',
                ".." => 'i',
                ".___" => 'j',
                "_._" => 'k',
                "._.." => 'l',
                "__" => 'm',
                "_." => 'n',
                "___" => 'o',
                ".__." => 'p',
                "__._" => 'q',
                "._." => 'r',
                "..." => 's',
                "_" => 't',
                ".._" => 'u',
                "..._" => 'v',
                ".__" => 'w',
                "_.._" => 'x',
                "_.__" => 'y',
                "__.." => 'z',
                "_____" => '0',
                ".____" => '1',
                "..___" => '2',
                "...__" => '3',
                "...._" => '4',
                "....." => '5',
                "_...." => '6',
                "__..." => '7',
                "___.." => '8',
                "____." => '9',
                "._._._" => '.',
                "__..__" => ',',
                "..__.." => '?',
                ".____." => '\'',
                "_._.__" => '!',
                "_.._." => '/',
                "_.__." => '(',
                "_.__._" => ')',
                "._..." => '&',
                "___..." => ':',
                "_._._." => ';',
                "_..._" => '=',
                "._._." => '+',
                "_...._" => '-',
                "..__._" => '_',
                "._.._." => '"',
                "..._.._" => '$',
                ".__._." => '@',
                "/" => ' ',
                _ => { error_values.push(c.to_string()); '?' }
            };
            result.push(letter);
        }
        result.push(' ');
    }
    result.pop();
    if error_values.len() == 0 {
        Ok(result)
    } else {
        Err(TranslationError {unsupported_characters: error_values, result: result})
    }
}

#[test]
fn decode_lower_case_letters() {
    assert_eq!("a", decode("._").unwrap());
    assert_eq!("b", decode("_...").unwrap());
    assert_eq!("c", decode("_._.").unwrap());
    assert_eq!("d", decode("_..").unwrap());
    assert_eq!("e", decode(".").unwrap());
    assert_eq!("f", decode(".._.").unwrap());
    assert_eq!("g", decode("__.").unwrap());
    assert_eq!("h", decode("....").unwrap());
    assert_eq!("i", decode("..").unwrap());
    assert_eq!("j", decode(".___").unwrap());
    assert_eq!("k", decode("_._").unwrap());
    assert_eq!("l", decode("._..").unwrap());
    assert_eq!("m", decode("__").unwrap());
    assert_eq!("n", decode("_.").unwrap());
    assert_eq!("o", decode("___").unwrap());
    assert_eq!("p", decode(".__.").unwrap());
    assert_eq!("q", decode("__._").unwrap());
    assert_eq!("r", decode("._.").unwrap());
    assert_eq!("s", decode("...").unwrap());
    assert_eq!("t", decode("_").unwrap());
    assert_eq!("u", decode(".._").unwrap());
    assert_eq!("v", decode("..._").unwrap());
    assert_eq!("w", decode(".__").unwrap());
    assert_eq!("x", decode("_.._").unwrap());
    assert_eq!("y", decode("_.__").unwrap());
    assert_eq!("z", decode("__..").unwrap());
}

#[test]
fn decode_numbers() {
    assert_eq!("1", decode(".____").unwrap());
    assert_eq!("2", decode("..___").unwrap());
    assert_eq!("3", decode("...__").unwrap());
    assert_eq!("4", decode("...._").unwrap());
    assert_eq!("5", decode(".....").unwrap());
    assert_eq!("6", decode("_....").unwrap());
    assert_eq!("7", decode("__...").unwrap());
    assert_eq!("8", decode("___..").unwrap());
    assert_eq!("9", decode("____.").unwrap());
    assert_eq!("0", decode("_____").unwrap());
}

#[test]
fn decode_other() {
    assert_eq!(".", decode("._._._").unwrap());
    assert_eq!(",", decode("__..__").unwrap());
    assert_eq!("?", decode("..__..").unwrap());
    assert_eq!("'", decode(".____.").unwrap());
    assert_eq!("!", decode("_._.__").unwrap());
    assert_eq!("/", decode("_.._.").unwrap());
    assert_eq!("(", decode("_.__.").unwrap());
    assert_eq!(")", decode("_.__._").unwrap());
    assert_eq!("&", decode("._...").unwrap());
    assert_eq!(":", decode("___...").unwrap());
    assert_eq!(";", decode("_._._.").unwrap());
    assert_eq!("=", decode("_..._").unwrap());
    assert_eq!("+", decode("._._.").unwrap());
    assert_eq!("-", decode("_...._").unwrap());
    assert_eq!("_", decode("..__._").unwrap());
    assert_eq!("\"", decode("._.._.").unwrap());
    assert_eq!("$", decode("..._.._").unwrap());
    assert_eq!("@", decode(".__._.").unwrap());
}

#[test]
fn decode_word() {
    assert_eq!("abc", decode("._ _... _._.").unwrap());
}

#[test]
fn decode_multiple_words() {
    assert_eq!("abc def", decode("._ _... _._. / _.. . .._.").unwrap());
}

#[test]
fn result_err() {
    let morse = match decode("_______ ... ___ ...") {
        Ok(x) => {
            assert!(false);
            x
        },
        Err(e) => {
            assert_eq!(e.unsupported_characters.len(), 1);
            assert_eq!(e.unsupported_characters[0], "_______");
            assert_eq!("?sos", e.result);
            e.result
        }
    };
    assert_eq!("?sos", morse);
}

#[test]
fn result_ok() {
    let morse = match decode("... ___ ...") {
        Ok(x) => {
            assert_eq!("sos", x);
            x
        },
        Err(e) => {
            assert!(false);
            e.result
        }
    };
    assert_eq!("sos", morse);
}