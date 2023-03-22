#[macro_use]
extern crate lazy_static;

pub mod solution;
pub mod tests;

use std::collections::HashMap;

lazy_static! {
    static ref MORSE_CODE: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("-....".to_string(), "6".to_string());
        m.insert("--..--".to_string(), ",".to_string());
        m.insert("-..-.".to_string(), "/".to_string());
        m.insert(".-".to_string(), "A".to_string());
        m.insert(".---".to_string(), "J".to_string());
        m.insert("...-".to_string(), "V".to_string());
        m.insert("...".to_string(), "S".to_string());
        m.insert("-.--.-".to_string(), ")".to_string());
        m.insert("--.".to_string(), "G".to_string());
        m.insert("..--..".to_string(), "?".to_string());
        m.insert("..-.".to_string(), "F".to_string());
        m.insert("-".to_string(), "T".to_string());
        m.insert("..".to_string(), "I".to_string());
        m.insert("--".to_string(), "M".to_string());
        m.insert("--..".to_string(), "Z".to_string());
        m.insert("....-".to_string(), "4".to_string());
        m.insert("....".to_string(), "H".to_string());
        m.insert(".".to_string(), "E".to_string());
        m.insert("-----".to_string(), "0".to_string());
        m.insert("----.".to_string(), "9".to_string());
        m.insert(".-...".to_string(), "&".to_string());
        m.insert("-....-".to_string(), "-".to_string());
        m.insert("...-..-".to_string(), "$".to_string());
        m.insert("-.-".to_string(), "K".to_string());
        m.insert("-.".to_string(), "N".to_string());
        m.insert(".-.-.-".to_string(), ".".to_string());
        m.insert(".-.-.".to_string(), "+".to_string());
        m.insert(".-..-.".to_string(), "\"".to_string());
        m.insert("-..-".to_string(), "X".to_string());
        m.insert(".----.".to_string(), "'".to_string());
        m.insert("--.-".to_string(), "Q".to_string());
        m.insert("...---...".to_string(), "SOS".to_string());
        m.insert(".--".to_string(), "W".to_string());
        m.insert("---..".to_string(), "8".to_string());
        m.insert("..--.-".to_string(), "_".to_string());
        m.insert("-.--.".to_string(), "(".to_string());
        m.insert(".----".to_string(), "1".to_string());
        m.insert("--...".to_string(), "7".to_string());
        m.insert("-.-.".to_string(), "C".to_string());
        m.insert("-...".to_string(), "B".to_string());
        m.insert(".-..".to_string(), "L".to_string());
        m.insert("-..".to_string(), "D".to_string());
        m.insert("-.--".to_string(), "Y".to_string());
        m.insert("---".to_string(), "O".to_string());
        m.insert(".--.".to_string(), "P".to_string());
        m.insert(".-.".to_string(), "R".to_string());
        m.insert("..-".to_string(), "U".to_string());
        m.insert("...--".to_string(), "3".to_string());
        m.insert(".....".to_string(), "5".to_string());
        m.insert("-.-.--".to_string(), "!".to_string());
        m.insert("---...".to_string(), ",".to_string());
        m.insert("-...-".to_string(), "=".to_string());
        m.insert(".--.-.".to_string(), "@".to_string());
        m.insert("..---".to_string(), "2".to_string());
        m.insert("-.-.-.".to_string(), ";".to_string());
        m
    };
}

fn main() {}
