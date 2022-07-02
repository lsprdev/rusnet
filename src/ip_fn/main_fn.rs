pub fn get_class(ip: String) -> String{
    let ip_num: i32 = ip.trim()
                        .parse()
                        .unwrap();
    let mut res = String::new();

    if ip_num >= 1 && ip_num <= 127 {
        res = "A".to_string();
    }
    else if ip_num >= 128 && ip_num <= 191 {
        res = "B".to_string();
    }
    else if ip_num >= 192 && ip_num <= 223 {
        res = "C".to_string();
    }
    else {
        res = "Invalid IP".to_string();
    }
    res
}

pub fn mask(mask: String) -> u16 {
    let mask_num: u16 = mask.trim()
                            .parse()
                            .unwrap();
    match mask_num {
        24 => 0,
        25 => 128,
        26 => 192,
        27 => 224,
        28 => 240,
        29 => 248,
        30 => 252,
        _ => 000,
    }
}

pub fn broadcast(mask: u16) -> u16  {

    let broadcast;
    match mask {
        0 => broadcast = (256-0)-1,
        128 => broadcast = (256-128)-1,
        192 => broadcast = (256-192)-1,
        224 => broadcast = (256-224)-1,
        240 => broadcast = (256-240)-1,
        248 => broadcast = (256-248)-1,
        252 => broadcast = (256-252)-1,
        _ => broadcast = 0,
    }
    broadcast
}

pub fn ip_range(last_octet: u8, broadcast: u16) -> Vec<u32> {
    let range: Vec<u8>;
    range = vec![last_octet+1, (broadcast-1).try_into().unwrap()];
    range.iter()
         .map(|&x| x as u32)
         .collect()
}

pub fn ip(ip: String) -> u8{
    let ip_num = ip.trim()
                    .parse()
                    .unwrap();
    ip_num
}
