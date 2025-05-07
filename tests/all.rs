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

#[test]
#[cfg(feature = "spanish")]
fn use_encode_spanish() {
    match morse::encode::encode("El veloz murciélago hindú comía feliz cardillo y kiwi. La cigüeña ¡tocaba el saxofón detrás del palenque de paja!.") {
        Ok(x) => println!("{}", x),
        Err(e) => {
            println!("The following chars were unsupported {:?}",
                     e.unsupported_characters)
        }
    }
}

#[test]
#[cfg(feature = "spanish")]
fn use_decode_spanish() {
    match morse::decode::decode(". ._.. / ..._ . ._.. ___ __.. / __ .._ ._. _._. .. .._.. ._.. ._ __. ___ / .... .. _. _.. .._ / _._. ___ __ .. ._ / .._. . ._.. .. __.. / _._. ._ ._. _.. .. ._.. ._.. ___ / _.__ / _._ .. .__ .. ._._._ / ._.. ._ / _._. .. __. ..__ . __.__ ._ / __..._ _ ___ _._. ._ _... ._ / . ._.. / ... ._ _.._ ___ .._. ___. _. / _.. . _ ._. .__._ ... / _.. . ._.. / .__. ._ ._.. . _. __._ .._ . / _.. . / .__. ._ .___ ._ _._.__ ._._._") {
        Ok(x) => println!("{}", x),
        Err(e) => {
            println!("The following chars were unsupported {:?}",
                     e.unsupported_characters)
        }
    }
}