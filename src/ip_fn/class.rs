pub fn class(fist_octet: u16) -> String {
    let fo = fist_octet;
    match fo {
        1 ..= 126 => "A".to_string(),
        128 ..= 191 => "B".to_string(),
        192 ..= 223 => "C".to_string(),
        _ => "E".to_string(),
    }

}