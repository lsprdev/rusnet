mod ip_fn;
use std::env;

fn main() {
    let args: Vec<String> = env::args()
                                    .collect();
    let ip: Vec<&str> = args[1]
                            .split(".")
                            .collect();
    let mask: String = args[2]
                            .split("/")
                            .collect();
    let mask_parsed: u16 = mask
                            .parse()
                            .unwrap();

    let mut ip_parsed: Vec<u16> = vec![];
    for i in 0..ip.len() {
        let ip_int: u16 = ip[i]
                            .parse()
                            .unwrap();
        ip_parsed.push(ip_int);

    }

    println!("Net Address => {}.{}.{}.{}", 
    ip_parsed[0], ip_parsed[1], ip_parsed[2], ip_parsed[3]);
    
    let sn = ip_fn::subnet::sn(mask_parsed);
    println!("Subnet Mask => {}.{}.{}.{}", 
    sn[0], sn[1], sn[2], sn[3]);

    let bc = ip_fn::broadcast::bc(sn[3]);
    println!("Broadcast Address => {}.{}.{}.{}", 
    ip_parsed[0], ip_parsed[1], ip_parsed[2], bc);

    let cl = ip_fn::class::class(ip_parsed[0]);
    println!("Standard Class => {}", cl);

    let rg = ip_fn::range::rg(ip_parsed[3], sn[3]);
    println!("Range => {} ~ {}", 
    rg[0], rg[1]);

    println!("Net Address Binary => {:b}.{:b}.{:b}.{:b}", 
    ip_parsed[0], ip_parsed[1], ip_parsed[2], ip_parsed[3]);

    println!("Mask Binary => {:b}.{:b}.{:b}.{:b}", 
    sn[0], sn[1], sn[2], sn[3]);

    println!("Broadcast Address Binary => {:b}.{:b}.{:b}.{:b}", 
    ip_parsed[0], ip_parsed[1], ip_parsed[2], bc);


}