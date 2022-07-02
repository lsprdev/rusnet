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
    println!("========================================");
    println!("IP Address => {}.{}.{}.{}", 
    ip(oct1.clone()).trim(), 
    ip(oct2.clone()).trim(), 
    ip(oct3.clone()).trim(), 
    ip(oct4.clone()).trim());
    
    println!("Subnet Mask => ", );
    println!("Net Address => ",);
    println!("Broadcast Address => ");
    println!("Standard Class => {}", get_class(oct1.clone()));
    println!("Range => ", );
    println!("IP Binary => ", );
    println!("Mask Binary => ", );
    println!("Net Address Binary => ", );
    println!("Broadcast Address Binary => ", );
}
