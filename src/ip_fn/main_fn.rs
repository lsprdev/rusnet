pub fn get_class(ip: String) -> String{
    let ip_num: i32 = ip.trim()
                        .parse()
                        .unwrap();

    let mut res = String::new();

    if ip_num >= 1 && ip_num <= 127 {
        res = "A".to_string();
    }
    res
}

pub fn ip(ip: String) -> String{
    ip
}
