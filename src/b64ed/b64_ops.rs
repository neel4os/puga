use base64::{engine::general_purpose::STANDARD, Engine as _};
pub fn encode(text:&String)->String{
    return STANDARD.encode(text)
}

pub fn decode(text:&String) ->String {
    let response = STANDARD.decode(text).expect("could not able to decode");
    return String::from_utf8(response).expect("invalid utf8 character in decoded string")
}