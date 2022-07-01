// IP Address =>
// Subnet Mask =>
// Net Address =>
// Broadcast Address =>
// Standard Class =>
// Range =>
// IP Binary =>
// Mask Binary =>
// Net Address Binary =>
// BC Address Binary =>

//Regex 

use std::io::{self, Write};
mod ip_fn;

fn main() {
    let ( mut oct0, mut oct1, mut oct2, mut oct3, mut mask ) = ( String::new(), String::new(), String::new(), String::new(), String::new() );

    print!("0 Octet => ");
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut oct0)
        .unwrap();

    print!("1 Octet => ");
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut oct1)
        .unwrap();

    print!("2 Octet => ");
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut oct2)
        .unwrap();
    
    print!("3 Octet => ");
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut oct3)
        .unwrap();

    print!("Mask => /");
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut mask)
        .unwrap();

    let ip = ip_fn::main_fn::ip;

    println!("========================================");
    println!("IP Address => {}.{}.{}.{}", ip(oct0.clone()).trim(), ip(oct1.clone()).trim(), ip(oct2.clone()).trim(), ip(oct3.clone()).trim());
    println!("Subnet Mask => ", );
    println!("Net Address => ",);
    println!("Broadcast Address => ");
    println!("Standard Class => ", );
    println!("Range => ", );
    println!("IP Binary => ", );
    println!("Mask Binary => ", );
    println!("Net Address Binary => ", );
    println!("Broadcast Address Binary => ", );
}
