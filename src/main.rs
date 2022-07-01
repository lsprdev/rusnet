
#![allow(dead_code)]

use std::io::{self, Write};
mod ip_functions;

fn main() {
    
    let ( mut ip, mut mask ) = ( String::new(), String::new() );

    print!("IP Address => ");
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut ip)
        .unwrap();

    print!("Mask => ");
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut mask)
        .unwrap();

    ip_functions::get_all::run(ip);
}
