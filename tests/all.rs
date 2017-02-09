extern crate morse;

#[test]
fn use_encode() {
    match morse::encode::encode("Hello World!") {
        Ok(x) => println!("{}", x),
        Err(e) => {
            println!("The following chars were unsupported {:?}",
                     e.unsupported_characters)
        }
    }
}

#[test]
fn use_decode() {
    match morse::decode::decode("... ___ ...") {
        Ok(x) => println!("{}", x),
        Err(e) => {
            println!("The following chars were unsupported {:?}",
                     e.unsupported_characters)
        }
    }
}
