pub fn bc(last_octet_subnet: u16) -> u16 {
    let qt_hosts = 256;
    let broadcast_address = (qt_hosts - last_octet_subnet)-1;
    broadcast_address
}