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

use std::io::{self, Write};
mod ip_fn;

fn main() {
    let ( mut ip, mut mask ) = ( String::new(), String::new() );

    print!("IP Address => ");
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut ip)
        .unwrap();

    print!("Mask => /");
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut mask)
        .unwrap();

    println!("========================================");
    println!("IP Address => {}", ip_fn::main_fn::ip(ip.clone()));
    println!("Subnet Mask => {}", ip_fn::main_fn::ip(ip.clone()));
    println!("Net Address => {}", ip_fn::main_fn::ip(ip.clone()));
    println!("Broadcast Address => ", );
    println!("Standard Class => ", );
    println!("Range => ", );
    println!("IP Binary => ", );
    println!("Mask Binary => ", );
    println!("Net Address Binary => ", );
    println!("Broadcast Address Binary => ", );
}
