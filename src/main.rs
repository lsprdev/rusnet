use std::io::{self, Write};
mod ip_fn;

fn main() {
    let ( mut oct1, mut oct2, mut oct3, mut oct4, mut mask ) = ( String::new(), String::new(), String::new(), String::new(), String::new() );

    print!("1st Octet => ");
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut oct1)
        .unwrap();

    print!("2nd Octet => ");
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut oct2)
        .unwrap();

    print!("3rd Octet => ");
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut oct3)
        .unwrap();
    
    print!("4th Octet => ");
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut oct4)
        .unwrap();

    print!("Mask => /");
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut mask)
        .unwrap();

    // ============================================================
    let ip = ip_fn::main_fn::ip;
    let get_class = ip_fn::main_fn::get_class;
    let get_mask = ip_fn::main_fn::mask;
    let get_broadcast = ip_fn::main_fn::broadcast;
    let get_ip_range = ip_fn::main_fn::ip_range;
    println!("========================================");
    
    println!("Net Address => {}.{}.{}.{}", 
    ip(oct1.clone()), 
    ip(oct2.clone()), 
    ip(oct3.clone()), 
    ip(oct4.clone()));
    
    println!("Subnet Mask => 255.255.255.{}", get_mask(mask.clone()));
    
    println!("Broadcast Address => {}.{}.{}.{}", 
    ip(oct1.clone()),
    ip(oct2.clone()),
    ip(oct3.clone()),
    get_broadcast(get_mask(mask.clone())));

    println!("Standard Class => {}", get_class(oct1.clone()));

    println!("Range => {} ~ {}", 
    get_ip_range(ip(oct4.clone()), get_broadcast(get_mask(mask.clone())))[0],
    get_ip_range(ip(oct4.clone()), get_broadcast(get_mask(mask.clone())))[1]);

    println!("IP Binary => ", );

    println!("Mask Binary => ", );

    println!("Net Address Binary => ", );

    println!("Broadcast Address Binary => ", );
}
