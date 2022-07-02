pub fn get_class(ip: String) -> String{
    let ip_num: i32 = ip.trim()
                        .parse()
                        .unwrap();
    
    let mut res = String::new();
    res.push_str("");
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
pub fn mask(mask: String) -> Vec<u16> {
    let mask_num: u16 = mask.trim()
                            .parse()
                            .unwrap();

    if mask_num >= 24 && mask_num <= 30 {
        let mut net: Vec<u16> = vec![255, 255, 255];
        match mask_num {
            24 => net.push(0),
            25 => net.push(128),
            26 => net.push(192),
            27 => net.push(224),
            28 => net.push(240),
            29 => net.push(248),
            30 => net.push(252),
            _ => net.push(000),
        }
        net
    }
    else {
        vec![0, 0, 0, 0]
    }
}

pub fn broadcast(mask: u16) -> u16  {
    let broadcast;
    let max_hosts: u16 = 256;
    match mask {
        0 => broadcast = (max_hosts-0)-1,
        128 => broadcast = (max_hosts-128)-1,
        192 => broadcast = (max_hosts-192)-1,
        224 => broadcast = (max_hosts-224)-1,
        240 => broadcast = (max_hosts-240)-1,
        248 => broadcast = (max_hosts-248)-1,
        252 => broadcast = (max_hosts-252)-1,
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
