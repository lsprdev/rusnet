mod ip_fn;
mod info;
use std::env;

/*  rusnet -- IPv4 Subnet Calculator
    Copyright (C) 2022  Gabriel L. Pereira.

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.

 */

/*
 *   author Gabriel L. Pereira <ogabrielpereira@pm.me>
 *   version 0.1.0
 *   since Jul 1, 2022
 */

fn main() {
    // Get the arguments from the command line
    let args: Vec<String> = env::args()
                                    .collect();

    // Split the first argument into a vector(".")
    let ip: Vec<&str> = args[1]
                            .split(".")
                            .collect();

    // Split the second argument("/")
    let mask: String = args[2]
                            .split("/")
                            .collect();

    // Parse the mask into a u16 -- String to u16
    let mask_parsed: u16 = mask
                            .parse()
                            .unwrap();

    // Parse the ip into a vector of u16 -- Vec<&str> to Vec<u16>
    let mut ip_parsed: Vec<u16> = vec![];
    for i in 0..ip.len() {
        let ip_u16: u16 = ip[i]
                            .parse()
                            .unwrap();
        ip_parsed.push(ip_u16);

    }
    println!("Net Address => {}.{}.{}.{}", 
    ip_parsed[0], ip_parsed[1], ip_parsed[2], ip_parsed[3]);
    
    let subnet = ip_fn::subnet::sn(mask_parsed);
    println!("Subnet Mask => {}.{}.{}.{}", 
    subnet[0], subnet[1], subnet[2], subnet[3]);

    let broadcast = ip_fn::broadcast::bc(subnet[3]);
    println!("Broadcast Address => {}.{}.{}.{}", 
    ip_parsed[0], ip_parsed[1], ip_parsed[2], broadcast);

    let class = ip_fn::class::class(ip_parsed[0]);
    println!("Standard Class => {}", class);

    let range = ip_fn::range::rg(ip_parsed[3], subnet[3]);
    println!("Range => {} ~ {}", 
    range[0], range[1]);

    println!("Net Address Binary => {:08b}.{:08b}.{:08b}.{:08b}", 
    ip_parsed[0], ip_parsed[1], ip_parsed[2], ip_parsed[3]);

    println!("Mask Binary => {:08b}.{:08b}.{:08b}.{:08b}", 
    subnet[0], subnet[1], subnet[2], subnet[3]);

    println!("Broadcast Address Binary => {:08b}.{:08b}.{:08b}.{:08b}", 
    ip_parsed[0], ip_parsed[1], ip_parsed[2], broadcast);


}