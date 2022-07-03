pub fn rg(last_octet: u16, last_octet_subnet: u16) -> Vec<u16>{

    let qt_hosts = 256;
    let mut range: Vec<u16> = vec![];
    
    // Start of the range
    let first_range = last_octet + 1;
    range.push(first_range);

    // End of the range
    let last_range = (qt_hosts - last_octet_subnet)-2;
    range.push(last_range);

    range
}